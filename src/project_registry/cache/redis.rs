use {
    super::{Cache, Output, ProjectData, Result},
    crate::project_registry::cache,
    anyhow::Context as _,
    async_trait::async_trait,
    deadpool_redis::{redis::AsyncCommands as _, Connection, Pool, Runtime},
};

const TTL_SECS: usize = 300;

#[derive(Clone)]
struct Adapter {
    redis_conn_pool: Pool,
}

pub fn new(url: impl Into<String>) -> Result<impl Cache> {
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
impl Cache for Adapter {
    async fn set(&self, project_id: &str, data: &Option<ProjectData>) -> Result<()> {
        let bytes = cache::serialize_data(data).context("Failed to serialize ProjectData")?;

        self.get_conn()
            .await?
            .set_ex(project_id, bytes, TTL_SECS)
            .await
            .context("SETEX operation failed")
    }

    async fn get(&self, project_id: &str) -> Result<Output> {
        // Careful: Specifying `Vec<u8>` here instead of `Option<Vec<u8>>` works as well
        // (IMO it shouldn't).
        let output: Option<Vec<u8>> = self
            .get_conn()
            .await?
            .get(project_id)
            .await
            .context("GET operation failed")?;

        Ok(match output {
            Some(bytes) => cache::deserialize_data(bytes.as_slice())
                .map(Output::Hit)
                .context("Failed to deserialize ProjectData")?,
            None => Output::Miss,
        })
    }
}
