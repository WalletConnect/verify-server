use {
    super::{Result, ScamGuard},
    crate::IsScam,
    anyhow::Context as _,
    async_trait::async_trait,
    reqwest::Url,
    serde::Deserialize,
    std::sync::Arc,
    tracing::warn,
};

const API_KEY_HEADER: &str = "x-api-key";

#[derive(Clone, Debug)]
struct Adapter {
    client: reqwest::Client,

    url: Arc<str>,
    key: Arc<str>,
}

pub fn new(url: String, key: String) -> impl ScamGuard {
    Adapter {
        client: reqwest::Client::new(),
        url: url.into(),
        key: key.into(),
    }
}

#[derive(Deserialize)]
struct ResponseBody {
    is_scam: bool,
}

#[async_trait]
impl ScamGuard for Adapter {
    async fn is_scam(&self, origin: &str) -> Result<IsScam> {
        let url = Url::parse(origin).ok();
        let Some(host) = url.as_ref().and_then(|url| url.host_str()) else {
            warn!("Origin({origin}) isn't a valid URL, skipping the scam check");
            return Ok(IsScam::Unknown);
        };

        let url = format!("{}/domain?domain={host}", self.url.as_ref());
        let req = self
            .client
            .get(url)
            .header(API_KEY_HEADER, self.key.as_ref());

        let resp = req.send().await.context("data API request failed")?;
        if resp.status() == 404 {
            return Ok(IsScam::Unknown);
        }

        let body: ResponseBody = resp
            .json()
            .await
            .context("failed to deserialize respone body as JSON")?;

        Ok(match body.is_scam {
            true => IsScam::Yes,
            false => IsScam::No,
        })
    }
}
