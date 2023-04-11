use {
    super::{ProjectData, ProjectRegistry, Result},
    async_trait::async_trait,
    cerberus::registry::{RegistryClient, RegistryHttpClient},
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
        Ok(RegistryClient::project_data(self, id).await?)
    }
}
