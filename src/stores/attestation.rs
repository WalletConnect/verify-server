use {
    super::StoreError,
    crate::{
        handlers::attestation::AttestationBody,
        stores::{self, StoreError::NotFound},
    },
    async_trait::async_trait,
    deadpool_redis::{
        redis::{cmd, AsyncCommands, FromRedisValue},
        Config,
        Runtime,
    },
    std::{sync::Arc, time::Duration},
    tracing::error,
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
        self.get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .set(id, origin)
            .await?;
        self.get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .expire::<_, usize>(id, Duration::from_secs(300).as_millis() as usize)
            .await?;
        Ok(())
    }

    async fn get_attestation(&self, id: &str) -> stores::Result<String> {
        let origin = self
            .get()
            .await
            .map_err(|e| StoreError::Cache(e.into()))?
            .get(id)
            .await?;
        Ok(origin)
    }
}
