use {
    anyhow::{anyhow, Context as _},
    axum::response::{IntoResponse, Response},
    axum_prometheus::{
        metrics_exporter_prometheus::{Matcher as MetricMatcher, PrometheusBuilder},
        AXUM_HTTP_REQUESTS_DURATION_SECONDS,
    },
    bouncer::{
        project_registry::{self, CachedExt as _},
        util::redis,
        Domain,
    },
    build_info::VersionControl,
    futures::{future::select, FutureExt},
    hyper::StatusCode,
    once_cell::sync::Lazy,
    serde::{Deserialize, Deserializer},
    std::{future::Future, str::FromStr},
    tap::TapFallible,
    tempfile::tempfile,
    tokio::signal::unix::{signal, SignalKind},
    tracing::{error, info},
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

    /// Additional domains to allow the Enclave to be served on.
    ///
    /// Intended for testing purposes on dev environments.
    #[serde(default)]
    pub domain_whitelist: Vec<Domain>,
}

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

    // By default `axum_prometheus` exposes http latency as a Summary, which
    // provides quite limited querying functionaliy. `set_buckets_for_metrics`
    // makes it a Histogram.
    let prometheus = PrometheusBuilder::new()
        .set_buckets_for_metric(
            MetricMatcher::Full(AXUM_HTTP_REQUESTS_DURATION_SECONDS.to_string()),
            axum_prometheus::SECONDS_DURATION_BUCKETS,
        )
        .context("Failed to set Prometheus buckets for HTTP request latency metrics")?
        .install_recorder()
        .context("Failed to install Prometheus metrics recorder")?;

    let attestation_store = redis::new("attestation_store", config.attestation_cache_url.clone())
        .context("Failed to initialize AttestationStore")?;

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

    let app = bouncer::new(
        config.domain_whitelist,
        (attestation_store, project_registry),
    );

    bouncer::http_server::run(
        app,
        config.port,
        move || prometheus.render(),
        config.prometheus_port,
        health_provider,
        signals,
    )
    .await;

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

fn health_provider() -> Response {
    fn checks() -> Result<(), anyhow::Error> {
        const MAX_USED_MEMORY_PERCENT: f32 = 75.0;

        let memory = sys_info::mem_info().context("Failed to get memory info")?;
        let used_memory_perc = (memory.total - memory.avail) as f32 / memory.total as f32 * 100.0;
        if used_memory_perc > MAX_USED_MEMORY_PERCENT {
            return Err(anyhow!(
                "Memory usage is critical! {:.2}%",
                used_memory_perc
            ));
        }

        tempfile()
            .map(drop)
            .context("Failed to create a temp file, seems like the disk isn't operational")
    }

    let status_code = checks()
        .map(|_| StatusCode::OK)
        .tap_err(|e| error!("Health check failed: {e:?}"))
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    static BODY: Lazy<String> = Lazy::new(|| {
        build_info::build_info!(fn build_info);

        let build_info = build_info();
        let name = &build_info.crate_info.name;
        let version = &build_info.crate_info.version;

        let Some(git) = build_info.version_control.as_ref().and_then(VersionControl::git) else {
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
    });

    (status_code, BODY.clone()).into_response()
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
