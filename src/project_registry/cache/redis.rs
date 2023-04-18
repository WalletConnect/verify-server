use {
    super::{Cache, Output, ProjectData, Result},
    crate::{project_registry::cache, util::redis},
    anyhow::Context as _,
    async_trait::async_trait,
};

const TTL_SECS: usize = 300;

#[async_trait]
impl Cache for redis::Adapter {
    async fn set_data(&self, project_id: &str, data: &Option<ProjectData>) -> Result<()> {
        let bytes = cache::serialize_data(data).context("Failed to serialize ProjectData")?;
        self.set_ex(project_id, bytes, TTL_SECS).await
    }

    async fn get_data(&self, project_id: &str) -> Result<Output> {
        let output: Option<Vec<u8>> = self.get(project_id).await?;
        Ok(match output {
            Some(bytes) => cache::deserialize_data(bytes.as_slice())
                .map(Output::Hit)
                .context("Failed to deserialize ProjectData")?,
            None => Output::Miss,
        })
    }
}
