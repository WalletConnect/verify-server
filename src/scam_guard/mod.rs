pub mod data_api;

use {
    crate::{
        cache::{self, Cache, Cached},
        IsScam,
    },
    async_trait::async_trait,
    metrics::counter,
    tap::TapFallible as _,
    tracing::{debug, error, instrument},
};

#[async_trait]
pub trait ScamGuard: Send + Sync + 'static {
    /// Checks whether the provided domain is a scam dApp or not.
    async fn is_scam(&self, domain: &str) -> Result<IsScam>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl<S, C> ScamGuard for Cached<S, C>
where
    S: ScamGuard,
    for<'a> C: Cache<&'a str, IsScam>,
{
    #[instrument(level = "debug", skip(self))]
    async fn is_scam(&self, domain: &str) -> Result<IsScam> {
        match self.cache.get(domain).await {
            Ok(cache::Output::Hit(data)) => {
                debug!("get: hit");
                counter!("scam_guard_cache_hits", 1);
                return Ok(data);
            }
            Ok(cache::Output::Miss) => {
                debug!("get: miss");
                counter!("scam_guard_cache_misses", 1);
            }
            Err(e) => {
                error!("get: {e:?}");
                counter!("scam_guard_cache_errors", 1);
            }
        };

        let data = self.inner.is_scam(domain).await?;

        let cache = self.cache.clone();
        let domain = domain.to_string();

        // Do not block on cache write.
        tokio::spawn(async move {
            let _ = cache
                .set(&domain, &data)
                .await
                .tap_err(|e| error!("set: {e:?}"))
                .tap_err(|_| counter!("scam_guard_cache_write_errors", 1))
                .tap_ok(|_| counter!("scam_guard_cache_writes", 1));
        });

        Ok(data)
    }
}
