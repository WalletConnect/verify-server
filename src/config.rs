use {serde::Deserialize, std::str::FromStr};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_is_test", skip)]
    /// This is an internal flag to disable logging, cannot be defined by user
    pub is_test: bool,

    // TELEMETRY
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,

    // Redis
    pub attestation_cache_url: String,
    pub project_registry_cache_url: String,

    pub project_registry_url: String,
    pub project_registry_auth_token: String,
}

impl Configuration {
    pub fn new() -> crate::Result<Configuration> {
        let config = envy::from_env::<Configuration>()?;
        Ok(config)
    }

    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }
}

fn default_port() -> u16 {
    3008
}

fn default_log_level() -> String {
    "WARN".to_string()
}

fn default_is_test() -> bool {
    false
}
