pub mod redis;

use {
    super::{ProjectData, Result},
    async_trait::async_trait,
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
    async fn get(&self, project_id: &str) -> Result<Option<ProjectData>>;
}
