use {
    super::{AttestationStore, Result},
    crate::util::redis,
    async_trait::async_trait,
};

const ATTESTATION_TTL_SECS: usize = 300;

#[async_trait]
impl AttestationStore for redis::Adapter {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        self.set_ex(id, origin, ATTESTATION_TTL_SECS).await
    }

    async fn get_attestation(&self, id: &str) -> Result<Option<String>> {
        self.get(id).await
    }
}
