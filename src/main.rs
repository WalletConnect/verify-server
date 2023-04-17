use {
    anyhow::Context as _,
    bouncer::{
        attestation_store,
        project_registry::{self, cache::MeteredExt as _, CachedExt as _},
    },
    serde::{Deserialize, Deserializer},
    std::str::FromStr,
    tokio::signal::unix::{signal, SignalKind},
};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_log_level")]
    #[serde(deserialize_with = "deserialize_log_level")]
    pub log_level: tracing::Level,

    #[serde(default)]
    pub log_pretty: bool,

    pub attestation_cache_url: String,

    pub project_registry_url: String,
    pub project_registry_auth_token: String,
    pub project_registry_cache_url: String,

    /// Indicates whether the service is being run in a dev environment.
    ///
    /// Setting this to `true` allows the Enclave to be served to
    /// `*.walletconnect.com`, `*.vercel.app` and `*.localhost` regardless
    /// of the verified domains of the project.
    #[serde(default)]
    pub is_dev: bool,
}

build_info::build_info!(fn build_info);

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = envy::from_env::<Configuration>()?;

    let sub = tracing_subscriber::fmt().with_max_level(config.log_level);
    if config.log_pretty {
        sub.pretty().init();
    } else {
        sub.json()
            .event_format(tracing_subscriber::fmt::format::json())
            .init();
    }

    let mut shutdown =
        signal(SignalKind::terminate()).context("Failed to install SIGTERM handler")?;

    let attestation_store = attestation_store::redis::new(config.attestation_cache_url.clone())
        .context("Failed to initialize AttestationStore")?;
    let cache = project_registry::cache::redis::new(config.project_registry_cache_url.clone())
        .context("Failed to initialize project_registry::Cache")?
        .metered();

    let project_registry = project_registry::cloud::new(
        config.project_registry_url.clone(),
        &config.project_registry_auth_token,
    )
    .context("Failed to initialize ProjectRegistry")?
    .cached(cache);

    let app = bouncer::new((attestation_store, project_registry), vec![]);

    bouncer::http_server::run(app, config.port, "".to_string(), shutdown.recv()).await;
    Ok(())
}

fn default_port() -> u16 {
    3008
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
