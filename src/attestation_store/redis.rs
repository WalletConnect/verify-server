use {
    super::{AttestationStore, Result},
    anyhow::Context as _,
    async_trait::async_trait,
    deadpool_redis::{redis::AsyncCommands as _, Connection, Pool, Runtime},
};

const ATTESTATION_TTL_SECS: usize = 300;

struct Adapter {
    redis_conn_pool: Pool,
}

pub fn new(url: impl Into<String>) -> Result<impl AttestationStore> {
    deadpool_redis::Config::from_url(url)
        .create_pool(Some(Runtime::Tokio1))
        .map(|redis_conn_pool| Adapter { redis_conn_pool })
        .context("Failed to create Redis connection pool")
}

impl Adapter {
    async fn get_conn(&self) -> Result<Connection> {
        self.redis_conn_pool
            .get()
            .await
            .context("Failed to get Redis connection from the pool")
    }
}

#[async_trait]
impl AttestationStore for Adapter {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        self.get_conn()
            .await?
            .set_ex(id, origin, ATTESTATION_TTL_SECS)
            .await
            .context("SETEX operation failed")
    }

    async fn get_attestation(&self, id: &str) -> Result<String> {
        self.get_conn()
            .await?
            .get(id)
            .await
            .context("GET operation failed")
    }
}
