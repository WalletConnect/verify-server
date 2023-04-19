use crate::Domain;

use {
    super::{ProjectData, ProjectRegistry, Result},
    async_trait::async_trait,
    cerberus::registry::{RegistryClient, RegistryHttpClient},
    metrics::counter,
    tap::{Tap, TapFallible},
};

pub fn new(base_url: impl Into<String>, auth_token: &str) -> Result<impl ProjectRegistry> {
    Ok(RegistryHttpClient::with_config(
        base_url,
        auth_token,
        Default::default(),
    )?)
}

#[async_trait]
impl ProjectRegistry for RegistryHttpClient {
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>> {
        let data = RegistryClient::project_data(self, id)
            .await
            .tap(|_| counter!("project_registry_requests", 1))
            .tap_err(|_| counter!("project_registry_errors", 1))?;
        let Some(data) = data else {
            return Ok(None);
        };

        let verified_domains = data.verified_domains.into_iter().map(Domain).collect();
        Ok(Some(ProjectData { verified_domains }))
    }
}
