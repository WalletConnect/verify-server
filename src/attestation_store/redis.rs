use {
    super::{AttestationStore, Result},
    crate::util::redis,
    anyhow::Context as _,
    async_trait::async_trait,
    metrics::counter,
    tap::TapFallible,
};

const ATTESTATION_TTL_SECS: usize = 300;

#[async_trait]
impl AttestationStore for redis::Adapter {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        self.set_ex(id, origin, ATTESTATION_TTL_SECS)
            .await
            .tap_err(|_| counter!("attestation_store_set_write_errors", 1))
            .context("SETEX operation failed")
    }

    async fn get_attestation(&self, id: &str) -> Result<Option<String>> {
        self.get(id)
            .await
            .tap_err(|_| counter!("attestation_store_get", 1))
            .context("GET operation failed")
    }
}
