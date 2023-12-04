pub mod cloud;

pub use cache::{Cache, Cached, CachedExt};
use {
    crate::{async_trait, cache, ProjectData, ProjectId},
    metrics::counter,
    tap::TapFallible as _,
    tracing::{debug, error, instrument},
};

#[async_trait]
pub trait ProjectRegistry: Send + Sync + 'static {
    async fn project_data(&self, id: &ProjectId) -> Result<Option<ProjectData>>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl<R, C> ProjectRegistry for Cached<R, C>
where
    R: ProjectRegistry,
    C: Cache<ProjectId, Option<ProjectData>>,
{
    #[instrument(level = "debug", skip(self))]
    async fn project_data(&self, id: &ProjectId) -> Result<Option<ProjectData>> {
        match self.cache.get(id).await {
            Ok(cache::Output::Hit(data)) => {
                debug!("get: hit");
                counter!("project_registry_cache_hits", 1);
                return Ok(data);
            }
            Ok(cache::Output::Miss) => {
                debug!("get: miss");
                counter!("project_registry_cache_misses", 1);
            }
            Err(e) => {
                error!("get: {e:?}");
                counter!("project_registry_cache_errors", 1);
            }
        };

        let data = self.inner.project_data(id).await?;

        let cache = self.cache.clone();
        let data_clone = data.clone();
        let id = *id;

        // Do not block on cache write.
        tokio::spawn(async move {
            let _ = cache
                .set(&id, &data_clone)
                .await
                .tap_err(|e| error!("set: {e:?}"))
                .tap_err(|_| counter!("project_registry_cache_write_errors", 1))
                .tap_ok(|_| counter!("project_registry_cache_writes", 1));
        });

        Ok(data)
    }
}
