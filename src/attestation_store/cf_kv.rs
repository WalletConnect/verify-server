use {
    super::{AttestationStore, Result},
    crate::http_server::{CsrfToken, TokenManager},
    async_trait::async_trait,
    hyper::StatusCode,
    reqwest::Url,
    serde::Serialize,
    std::time::Duration,
};

#[derive(Clone)]
pub struct CloudflareKv {
    pub endpoint: Url,
    pub token_manager: TokenManager,
    pub http_client: reqwest::Client,
}

impl CloudflareKv {
    pub fn new(endpoint: Url, token_manager: TokenManager) -> Self {
        Self {
            endpoint,
            token_manager,
            http_client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SetAttestationCompatBody<'a> {
    attestation_id: &'a str,
    origin: &'a str,
}

#[async_trait]
impl AttestationStore for CloudflareKv {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()> {
        let url = self.endpoint.join("/attestation")?;
        let res = self
            .http_client
            .post(url)
            .header(
                CsrfToken::header_name(),
                self.token_manager
                    .generate_csrf_token()
                    .map_err(|e| anyhow::anyhow!("{e:?}"))?,
            )
            .json(&SetAttestationCompatBody {
                attestation_id: id,
                origin,
            })
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
        let url = self
            .endpoint
            .join(&format!("/v1/compat-attestation/{id}"))?;
        let response = self
            .http_client
            .get(url)
            .timeout(Duration::from_secs(1))
            .send()
            .await?;
        if response.status().is_success() {
            let value = response.text().await?;
            Ok(Some(value))
        } else if response.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            Err(anyhow::anyhow!(
                "Failed to get attestation: status:{} response body:{:?}",
                response.status(),
                response.text().await
            ))
        }
    }
}
