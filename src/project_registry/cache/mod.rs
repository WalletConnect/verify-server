pub mod redis;

use {
    super::{ProjectData, Result},
    crate::{Counter, ProjectRegistry},
    async_trait::async_trait,
    std::sync::Arc,
    tap::TapFallible as _,
    tracing::{debug, error, info, instrument},
};

fn serialize_data(data: &Option<ProjectData>) -> Result<Vec<u8>> {
    Ok(rmp_serde::to_vec(data)?)
}

fn deserialize_data(bytes: &[u8]) -> Result<Option<ProjectData>> {
    Ok(rmp_serde::from_slice(bytes)?)
}

#[async_trait]
pub trait Cache: Clone + Send + Sync + 'static {
    async fn set(&self, project_id: &str, data: &Option<ProjectData>) -> Result<()>;
    async fn get(&self, project_id: &str) -> Result<Output>;
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
    #[instrument(skip(self))]
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>> {
        match self.cache.get(id).await {
            Ok(Output::Hit(data)) => {
                debug!("hit");
                return Ok(data);
            }
            Ok(Output::Miss) => info!("miss"),
            Err(e) => error!("Cache::get: {e:?}"),
        };

        let data = self.registry.project_data(id).await?;

        let cache = self.cache.clone();
        let id = id.to_string();
        let data_clone = data.clone();

        // Do not block on cache write.
        tokio::spawn(async move {
            cache
                .set(&id, &data_clone)
                .await
                .map_err(|e| error!("Cache::set: {e:?}"))
        });

        Ok(data)
    }
}
#[derive(Default)]
pub struct Metrics {
    hits: Counter,
    misses: Counter,
    writes: Counter,

    read_errors: Counter,
    write_errors: Counter,
}

#[derive(Clone)]
pub struct Metered<C> {
    cache: C,
    metrics: Arc<Metrics>,
}

pub trait MeteredExt: Sized {
    fn metered(self) -> Metered<Self> {
        Metered {
            cache: self,
            metrics: Arc::new(Metrics::default()),
        }
    }
}

impl<C: Cache> MeteredExt for C {}

impl<C> AsRef<Metrics> for Metered<C> {
    fn as_ref(&self) -> &Metrics {
        &self.metrics
    }
}

#[async_trait]
impl<C: Cache> Cache for Metered<C> {
    async fn set(&self, project_id: &str, data: &Option<ProjectData>) -> Result<()> {
        self.cache
            .set(project_id, data)
            .await
            .tap_ok(|_| self.metrics.writes.incr())
            .tap_err(|_| self.metrics.write_errors.incr())
    }

    async fn get(&self, project_id: &str) -> Result<Output> {
        self.cache
            .get(project_id)
            .await
            .tap_ok(|out| match out {
                Output::Hit(_) => self.metrics.hits.incr(),
                Output::Miss => self.metrics.misses.incr(),
            })
            .tap_err(|_| self.metrics.read_errors.incr())
    }
}
