pub mod cache;
pub mod cloud;

use crate::{async_trait, ProjectData};
pub use cache::{Cache, Cached, CachedExt};

#[async_trait]
pub trait ProjectRegistry: Send + Sync + 'static {
    async fn project_data(&self, id: &str) -> Result<Option<ProjectData>>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
