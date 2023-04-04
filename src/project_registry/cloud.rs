use async_trait::async_trait;
use cerberus::registry::RegistryHttpClient;

use super::{ProjectData, ProjectRegistry, Result};

#[derive(Debug)]
struct Proxy {
    cerberus: RegistryHttpClient,
    // TODO: cache
}

pub fn new(base_url: impl Into<String>, auth_token: &str) -> Result<impl ProjectRegistry> {
    Ok(Proxy {
        cerberus: RegistryHttpClient::with_config(base_url, auth_token, Default::default())?,
    })
}

#[async_trait]
impl ProjectRegistry for Proxy {
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>> {
        self.cerberus.project_data(id).await
    }
}
