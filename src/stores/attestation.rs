
use std::time::Duration;

use deadpool_redis::redis::AsyncCommands;
use tracing::error;

use crate::handlers::attestation::AttestationBody;

use super::StoreError;

use {
    crate::stores::{self, StoreError::NotFound},
    async_trait::async_trait,
    std::sync::Arc,
    deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime},
};

pub type AttestationStoreArc = Arc<dyn AttestationStore + Send + Sync + 'static>;

#[async_trait]
pub trait AttestationStore {
    async fn set_attestation(&self, id: &str, origin: &str) -> stores::Result<()>;
    async fn get_attestation(&self, id: &str) -> stores::Result<String>;
}

#[async_trait]
impl AttestationStore for deadpool_redis::Pool {
    async fn set_attestation(&self, id: &str, origin: &str) -> stores::Result<()> {
        self
            .get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .set(id, origin)
            .await?;
        self
            .get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .expire::<_, usize>(id, Duration::from_secs(300).as_millis() as usize)
            .await?;
        Ok(())
    }
    async fn get_attestation(&self, _id: &str) -> stores::Result<String> {
        let origin = self
            .get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .get::<_, String>("id")
            .await?;
        Ok(origin)
    }
}
