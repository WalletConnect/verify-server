use {
    super::{ProjectData, ProjectRegistry, Result},
    crate::{Domain, ProjectId},
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
    async fn project_data(&self, id: ProjectId) -> Result<Option<ProjectData>> {
        let data = RegistryClient::project_data(self, id.as_ref())
            .await
            .tap(|_| counter!("project_registry_requests", 1))
            .tap_err(|_| counter!("project_registry_errors", 1))?;
        let Some(data) = data else {
            return Ok(None);
        };

        Ok(Some(ProjectData {
            is_verify_enabled: data.is_verify_enabled,
            verified_domains: data.verified_domains.into_iter().map(Domain).collect(),
        }))
    }
}
