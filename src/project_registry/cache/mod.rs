use {
    super::{ProjectData, Result},
    crate::{async_trait, ProjectId, ProjectRegistry},
    metrics::counter,
    tap::TapFallible as _,
    tracing::{debug, error, instrument},
};

pub mod redis;

fn serialize_data(data: &Option<ProjectData>) -> Result<Vec<u8>> {
    Ok(rmp_serde::to_vec(data)?)
}

fn deserialize_data(bytes: &[u8]) -> Result<Option<ProjectData>> {
    Ok(rmp_serde::from_slice(bytes)?)
}

#[async_trait]
pub trait Cache: Clone + Send + Sync + 'static {
    async fn set_data(&self, project_id: ProjectId, data: &Option<ProjectData>) -> Result<()>;
    async fn get_data(&self, project_id: ProjectId) -> Result<Output>;
}

// Option<Option<_>> is gross and I just shot myself in the foot with it.
// TODO: Come up with a better name
pub enum Output {
    Hit(Option<ProjectData>),
    Miss,
}

pub trait CachedExt: Sized {
    fn cached<C: Cache>(self, cache: C) -> Cached<Self, C> {
        Cached {
            registry: self,
            cache,
        }
    }
}

impl<R: ProjectRegistry> CachedExt for R {}

pub struct Cached<R, C> {
    registry: R,
    cache: C,
}

#[async_trait]
impl<R, C> ProjectRegistry for Cached<R, C>
where
    R: ProjectRegistry,
    C: Cache,
{
    #[instrument(level = "debug", skip(self))]
    async fn project_data(&self, id: ProjectId) -> Result<Option<ProjectData>> {
        match self.cache.get_data(id).await {
            Ok(Output::Hit(data)) => {
                debug!("get: hit");
                counter!("project_registry_cache_hits", 1);
                return Ok(data);
            }
            Ok(Output::Miss) => {
                debug!("get: miss");
                counter!("project_registry_cache_misses", 1);
            }
            Err(e) => {
                error!("get: {e:?}");
                counter!("project_registry_cache_errors", 1);
            }
        };

        let data = self.registry.project_data(id).await?;

        let cache = self.cache.clone();
        let data_clone = data.clone();

        // Do not block on cache write.
        tokio::spawn(async move {
            let _ = cache
                .set_data(id, &data_clone)
                .await
                .tap_err(|e| error!("set: {e:?}"))
                .tap_err(|_| counter!("project_registry_cache_write_errors", 1))
                .tap_ok(|_| counter!("project_registry_cache_writes", 1));
        });

        Ok(data)
    }
}
