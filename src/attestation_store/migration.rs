use {
    super::{cf_kv::CloudflareKv, AttestationStore, Result},
    crate::util::redis,
    async_trait::async_trait,
};

pub struct MigrationStore {
    redis: redis::Adapter,
    cf_kv: CloudflareKv,
}

impl MigrationStore {
    pub fn new(redis: redis::Adapter, cf_kv: CloudflareKv) -> Self {
        Self { redis, cf_kv }
    }
}

#[async_trait]
impl AttestationStore for MigrationStore {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        let redis_fut = self.redis.set_attestation(id, origin);
        let cf_kv_fut = self.cf_kv.set_attestation(id, origin);
        tokio::try_join!(redis_fut, cf_kv_fut).map(|_| ())
    }

    async fn get_attestation(&self, id: &str) -> Result<Option<String>> {
        if let Some(attestation) = self.redis.get_attestation(id).await? {
            Ok(Some(attestation))
        } else {
            self.cf_kv.get_attestation(id).await
        }
    }
}
