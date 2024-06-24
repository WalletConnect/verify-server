use {
    super::{AttestationStore, Result},
    async_trait::async_trait,
    serde::Serialize,
    std::time::Duration,
};

const ATTESTATION_TTL_SECS: usize = 300;

pub struct CloudflareKv {
    pub account_id: String,
    pub namespace_id: String,
    pub bearer_token: String,
    pub http_client: reqwest::Client,
}

impl CloudflareKv {
    pub fn new(account_id: String, namespace_id: String, bearer_token: String) -> Self {
        Self {
            account_id,
            namespace_id,
            bearer_token,
            http_client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct SetBulkBody<'a> {
    expiration: usize,
    key: &'a str,
    value: &'a str,
}

#[async_trait]
impl AttestationStore for CloudflareKv {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{account_id}/storage/kv/namespaces/{namespace_id}/bulk",
            account_id = self.account_id, namespace_id = self.namespace_id
        );
        let res = self
            .http_client
            .put(&url)
            .bearer_auth(&self.bearer_token)
            .json(&vec![SetBulkBody {
                expiration: ATTESTATION_TTL_SECS,
                key: id,
                value: origin,
            }])
            .timeout(Duration::from_secs(1))
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to set attestation: status:{} response body:{:?}",
                res.status(),
                res.text().await
            ))
        }
    }

    async fn get_attestation(&self, id: &str) -> Result<Option<String>> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{id}",
            account_id = self.account_id, namespace_id = self.namespace_id
        );
        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .timeout(Duration::from_secs(1))
            .send()
            .await?;
        // TODO what is the status code for a key not found?
        // TODO for not-key not found errors throw error instead of None
        if response.status().is_success() {
            let value = response.text().await?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
