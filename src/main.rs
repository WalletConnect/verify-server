use {
    anyhow::Context as _,
    aws_config::{meta::region::RegionProviderChain, BehaviorVersion},
    aws_sdk_s3::{config::Region, Client as S3Client},
    axum_prometheus::{
        metrics_exporter_prometheus::{Matcher as MetricMatcher, PrometheusBuilder},
        utils::SECONDS_DURATION_BUCKETS,
        AXUM_HTTP_REQUESTS_DURATION_SECONDS,
    },
    bouncer::{
        attestation_store::{cf_kv::CloudflareKv, migration},
        event_sink,
        http_server::{RequestInfo, ServerConfig, TokenManager},
        project_registry::{self, CachedExt as _},
        scam_guard,
        util::redis,
        GetAttestationHandled,
        GetVerifyStatusHandled,
        IsScam,
        SetAttestationHandled,
        VerifyStatus,
    },
    build_info::VersionControl,
    futures::{future::select, FutureExt},
    parquet_derive::ParquetRecordWriter,
    serde::{Deserialize, Deserializer},
    std::{future::Future, str::FromStr, sync::Arc},
    tap::TapFallible,
    tokio::signal::unix::{signal, SignalKind},
    tracing::info,
    wc::geoip::MaxMindResolver,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_prometheus_port")]
    pub prometheus_port: u16,

    #[serde(default = "default_log_level")]
    #[serde(deserialize_with = "deserialize_log_level")]
    pub log_level: tracing::Level,

    #[serde(default)]
    pub log_pretty: bool,

    pub attestation_cache_url: String,

    pub project_registry_url: String,
    pub project_registry_auth_token: String,
    pub project_registry_cache_url: String,

    pub data_api_url: String,
    pub data_api_auth_token: String,
    pub scam_guard_cache_url: String,

    pub cf_kv_endpoint: String,

    pub secret: String,

    pub s3_endpoint: Option<String>,

    pub data_lake_bucket: Option<String>,

    pub geoip_db_bucket: Option<String>,
    pub geoip_db_key: Option<String>,

    pub blocked_countries: Vec<String>,
}

build_info::build_info!(fn build_info);

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = envy::from_env::<Configuration>().context("Failed to build config")?;

    let signals = shutdown_signals()?;

    let sub = tracing_subscriber::fmt().with_max_level(config.log_level);
    if config.log_pretty {
        sub.pretty().init();
    } else {
        sub.json()
            .event_format(tracing_subscriber::fmt::format::json())
            .init();
    }

    let s3_client = s3_client(&config).await;
    let geoip_resolver = geoip_resolver(&config, &s3_client).await;

    // By default `axum_prometheus` exposes http latency as a Summary, which
    // provides quite limited querying functionaliy. `set_buckets_for_metrics`
    // makes it a Histogram.
    let prometheus = PrometheusBuilder::new()
        .set_buckets_for_metric(
            MetricMatcher::Full(AXUM_HTTP_REQUESTS_DURATION_SECONDS.to_string()),
            SECONDS_DURATION_BUCKETS,
        )
        .context("Failed to set Prometheus buckets for HTTP request latency metrics")?
        .install_recorder()
        .context("Failed to install Prometheus metrics recorder")?;

    let attestation_store = {
        let redis_attestation_store =
            redis::new("attestation_store", config.attestation_cache_url.clone())
                .context("Failed to initialize AttestationStore")?;
        let cf_kv_attestation_store = CloudflareKv::new(
            config
                .cf_kv_endpoint
                .parse()
                .context("Failed to parse cf_kv_endpoint")?,
            TokenManager::new(config.secret.as_bytes()),
        );
        migration::Store::new(redis_attestation_store, cf_kv_attestation_store)
    };

    let project_registry_cache = redis::new(
        "project_registry_cache",
        config.project_registry_cache_url.clone(),
    )
    .context("Failed to initialize project_registry::Cache")?;

    let project_registry = project_registry::cloud::new(
        config.project_registry_url.clone(),
        &config.project_registry_auth_token,
    )
    .context("Failed to initialize ProjectRegistry")?
    .cached(project_registry_cache);

    let scam_guard_cache = redis::new("scam_guard_cache", config.scam_guard_cache_url.clone())
        .context("Failed to initialize scam_guard::Cache")?;

    let scam_guard = scam_guard::data_api::new(config.data_api_url, config.data_api_auth_token)
        .cached(scam_guard_cache);

    let event_sink = if let Some(bucket) = config.data_lake_bucket {
        Some(event_sink::s3::requests_dir(s3_client, bucket).await?)
    } else {
        tracing::info!("data_lake_bucket is not specified, analytics are going to be disabled");
        None
    };

    let svc = bouncer::Service::new((attestation_store, project_registry, scam_guard))
        .observable(event_sink);

    let server_cfg = ServerConfig {
        port: config.port,
        metrics_port: config.prometheus_port,
        secret: config.secret.as_bytes(),
        blocked_countries: config.blocked_countries,
    };

    bouncer::http_server::run(
        server_cfg,
        svc,
        move || prometheus.render(),
        health_provider,
        geoip_resolver,
        signals,
    )
    .await?;

    Ok(())
}

fn shutdown_signals() -> Result<impl Future, anyhow::Error> {
    let mut term = signal(SignalKind::terminate()).context("Failed to install SIGTERM handler")?;
    let mut int = signal(SignalKind::interrupt()).context("Failed to install SIGINT handler")?;

    Ok(select(
        Box::pin(async move { term.recv().map(|_| info!("SIGTERM received")).await }),
        Box::pin(async move { int.recv().map(|_| info!("SIGINT received")).await }),
    ))
}

fn health_provider() -> String {
    let build_info = build_info();
    let name = &build_info.crate_info.name;
    let version = &build_info.crate_info.version;

    let Some(git) = build_info
        .version_control
        .as_ref()
        .and_then(VersionControl::git)
    else {
        return format!("{} v{}", name, version);
    };

    format!(
        "{} v{}, commit: {}, timestamp: {}, branch: {}",
        name,
        version,
        git.commit_short_id,
        git.commit_timestamp,
        git.branch.as_deref().unwrap_or_default(),
    )
}

fn default_port() -> u16 {
    3000
}

fn default_prometheus_port() -> u16 {
    4000
}

fn default_log_level() -> tracing::Level {
    tracing::Level::INFO
}

fn deserialize_log_level<'de, D>(de: D) -> Result<tracing::Level, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error as _;

    let s = String::deserialize(de)?;
    tracing::Level::from_str(&s)
        .map_err(|e| D::Error::custom(format!("Invalid tracing::Level: {e}")))
}

pub async fn s3_client(config: &Configuration) -> S3Client {
    let region_provider = RegionProviderChain::first_try(Region::new("eu-central-1"));
    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let aws_config = match &config.s3_endpoint {
        Some(s3_endpoint) => {
            tracing::info!(%s3_endpoint, "initializing s3 client with custom endpoint");

            aws_sdk_s3::config::Builder::from(&shared_config)
                .endpoint_url(s3_endpoint)
                .build()
        }
        _ => aws_sdk_s3::config::Builder::from(&shared_config).build(),
    };

    S3Client::from_conf(aws_config)
}

async fn geoip_resolver(
    config: &Configuration,
    s3_client: &S3Client,
) -> Option<Arc<MaxMindResolver>> {
    match (&config.geoip_db_bucket, &config.geoip_db_key) {
        (Some(bucket), Some(key)) => {
            info!(%bucket, %key, "initializing geoip database from aws s3");

            MaxMindResolver::from_aws_s3(s3_client, bucket, key)
                .await
                .tap_err(|err| info!(?err, "failed to load geoip resolver"))
                .ok()
                .map(Arc::new)
        }
        _ => {
            info!("geoip lookup is disabled");
            None
        }
    }
}

#[derive(Debug, Default, ParquetRecordWriter)]
struct RequestRecord {
    r#type: &'static str,
    success: bool,

    project_id: Option<String>,
    verify_status: Option<&'static str>,
    attestation_id: Option<String>,
    origin: Option<String>,
    is_scam: Option<bool>,

    user_agent: Option<String>,
    country: Option<Arc<str>>,
}

impl<'c, 'r> From<GetVerifyStatusHandled<'c, 'r, RequestInfo>> for RequestRecord {
    fn from(ev: GetVerifyStatusHandled<'c, 'r, RequestInfo>) -> Self {
        Self {
            r#type: "get_verify_status",
            success: ev.result.is_ok(),
            project_id: Some(ev.cmd.inner.project_id.as_ref().to_string()),
            verify_status: ev.result.as_ref().ok().map(|status| match status {
                VerifyStatus::Disabled => "disabled",
                VerifyStatus::Enabled { .. } => "enabled",
            }),
            user_agent: ev.cmd.context.user_agent,
            country: ev.cmd.context.country,
            ..Default::default()
        }
    }
}

impl<'c, 'r> From<SetAttestationHandled<'c, 'r, RequestInfo>> for RequestRecord {
    fn from(ev: SetAttestationHandled<'c, 'r, RequestInfo>) -> Self {
        Self {
            r#type: "set_attestation",
            success: ev.result.is_ok(),
            attestation_id: Some(ev.cmd.inner.id.to_string()),
            origin: Some(ev.cmd.inner.origin.to_string()),
            user_agent: ev.cmd.context.user_agent,
            country: ev.cmd.context.country,
            ..Default::default()
        }
    }
}

impl<'c, 'r> From<GetAttestationHandled<'c, 'r, RequestInfo>> for RequestRecord {
    fn from(ev: GetAttestationHandled<'c, 'r, RequestInfo>) -> Self {
        let attestation = ev.result.as_ref().ok().and_then(|opt| opt.as_ref());

        Self {
            r#type: "get_attestation",
            success: ev.result.is_ok(),
            attestation_id: Some(ev.cmd.inner.id.to_string()),
            origin: attestation.map(|a| a.origin.to_string()),
            is_scam: attestation.and_then(|a| match a.is_scam {
                IsScam::Yes => Some(true),
                IsScam::No => Some(false),
                IsScam::Unknown => None,
            }),
            user_agent: ev.cmd.context.user_agent,
            country: ev.cmd.context.country,
            ..Default::default()
        }
    }
}
