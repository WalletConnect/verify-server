pub mod cache;
pub mod cloud;

pub use cache::{Cache, Cached, CachedExt};

use crate::{async_trait, ProjectData, ProjectId};

#[async_trait]
pub trait ProjectRegistry: Send + Sync + 'static {
    async fn project_data(&self, id: ProjectId) -> Result<Option<ProjectData>>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
