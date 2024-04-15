use {
    anyhow::Context as _,
    deadpool_redis::{
        redis::{AsyncCommands as _, FromRedisValue, ToRedisArgs},
        Connection,
        Pool,
        Runtime,
    },
    metrics::counter,
    tap::TapFallible,
};

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Adapter {
    db_name: &'static str,
    redis_conn_pool: Pool,
}

pub fn new(db_name: &'static str, url: impl Into<String>) -> Result<Adapter> {
    deadpool_redis::Config::from_url(url)
        .create_pool(Some(Runtime::Tokio1))
        .context("Failed to create Redis connection pool")
        .map(|redis_conn_pool| Adapter {
            db_name,
            redis_conn_pool,
        })
}

impl Adapter {
    fn incr_counter(&self, name: &'static str) {
        counter!(name, 1, "db" => self.db_name)
    }

    async fn get_conn(&self) -> Result<Connection> {
        self.redis_conn_pool
            .get()
            .await
            .tap_err(|_| self.incr_counter("redis_conn_errors"))
            .context("Failed to get Redis connection from the pool")
    }

    pub async fn set_ex<K, V>(&self, key: K, value: V, seconds: usize) -> Result<()>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        self.get_conn()
            .await?
            .set_ex(key, value, seconds)
            .await
            .context("SETEX operation failed")
            .tap_ok(|_| self.incr_counter("redis_writes"))
            .tap_err(|_| self.incr_counter("redis_write_errors"))
    }

    // Returning Option<V> instead of V seems much more reasonable.
    pub async fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: ToRedisArgs + Send + Sync,
        V: FromRedisValue,
    {
        self.get_conn()
            .await?
            .get(key)
            .await
            .context("GET operation failed")
            .tap_ok(|_| self.incr_counter("redis_reads"))
            .tap_err(|_| self.incr_counter("redis_read_errors"))
    }
}
