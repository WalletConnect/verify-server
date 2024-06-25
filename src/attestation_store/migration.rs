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
        let (redis_res, cf_kv_res) = tokio::join!(redis_fut, cf_kv_fut);
        if let Err(e) = cf_kv_res {
            log::error!("Failed to set attestation in Cloudflare KV: {e} {e:?}");
        }
        redis_res
    }

    async fn get_attestation(&self, id: &str) -> Result<Option<String>> {
        if let Some(attestation) = self.redis.get_attestation(id).await? {
            Ok(Some(attestation))
        } else {
            let res = self.cf_kv.get_attestation(id).await;
            match res {
                Ok(a) => Ok(a),
                Err(e) => {
                    log::error!("Failed to get attestation from Cloudflare KV: {e} {e:?}");
                    Ok(None)
                }
            }
        }
    }
}
