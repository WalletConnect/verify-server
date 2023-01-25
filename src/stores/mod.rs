pub mod attestation;

type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Not found error, params are entity name and identifier
    #[error("Cannot find {0} with specified identifier {1}")]
    NotFound(String, String),

    #[error("Failed to set ttl {0} for id {1}")]
    SetExpiry(String, String),

    #[error(transparent)]
    Cache(#[from] deadpool_redis::PoolError),

    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
}
