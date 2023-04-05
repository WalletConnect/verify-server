pub mod cloud;

pub mod cache;

use {async_trait::async_trait, tracing as log};
pub use {cache::Cache, cerberus::project::ProjectData};

#[async_trait]
pub trait ProjectRegistry: Send + Sync {
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

struct WithCaching<R, C> {
    registry: R,
    cache: C,
}

pub fn with_caching(registry: impl ProjectRegistry, cache: impl Cache) -> impl ProjectRegistry {
    WithCaching { registry, cache }
}

#[async_trait]
impl<R, C> ProjectRegistry for WithCaching<R, C>
where
    R: ProjectRegistry,
    C: Cache,
{
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>> {
        let cached = self
            .cache
            .get(id)
            .await
            .map_err(|e| log::error!("Failed to get ProjectData(id: {id}) from cache: {e}"))
            .unwrap_or_default();
        if let Some(data) = cached {
            return Ok(Some(data));
        }

        let data = self.registry.project_data(id).await?;
        let _ = self
            .cache
            .set(id, &data)
            .await
            .map_err(|e| log::error!("Failed to cache ProjectData(id: {id}): {e}"));

        Ok(data)
    }
}
