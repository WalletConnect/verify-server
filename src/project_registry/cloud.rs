use {
    super::{ProjectData, ProjectRegistry, Result},
    crate::{SecondLevelDomain, TopLevelDomain},
    anyhow::anyhow,
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
        let Some(data) = RegistryClient::project_data(self, id).await? else {
            return Ok(None);
        };

        let verified_domains = data
            .verified_domains
            .into_iter()
            .map(|d| try_parse_domain(&d).ok_or_else(|| anyhow!("Invalid domain: {d}")))
            .collect::<Result<_>>()?;

        Ok(Some(ProjectData { verified_domains }))
    }
}

fn try_parse_domain(s: &str) -> Option<(SecondLevelDomain, TopLevelDomain)> {
    let mut parts = s.split('.');
    let sld = parts.next()?;
    let tld = parts.next()?;

    match parts.next() {
        Some(_) => None,
        None => Some((sld.to_string().into(), tld.to_string().into())),
    }
}
