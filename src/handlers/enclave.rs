use {
    crate::{state::AppState, Infra, ProjectRegistry as _},
    axum::{
        extract::{Path, State},
        response::{Html, IntoResponse},
    },
    hyper::{header, StatusCode},
    std::sync::Arc,
    tracing as log,
};

const INDEX_HTML: &str = r#"
<!-- index.html -->
<html>
  <head>
      <script src="/index.js"></script>
  </head>
</html>
"#;

pub async fn handler(
    Path(path): Path<String>,
    State(state): State<Arc<AppState<impl Infra>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let body = match path.as_str() {
        "index.js" => download_iframe()
            .await
            .map_err(|e| log::error!("Failed to download iframe: {e}"))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .map(Html)?,
        id => state
            .project_registry()
            .project_data(id)
            .await
            .map_err(|e| log::error!("Failed to query ProjectData: {e}"))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map(|_| Html(INDEX_HTML.into()))
            .ok_or(StatusCode::NOT_FOUND)?,
    };

    let policy = "frame-ancestors https://react-app.walletconnect.com";

    Ok(([(header::CONTENT_SECURITY_POLICY, policy)], body))
}

// TODO: bundle it / download during initialization
async fn download_iframe() -> Result<String, DownloadIframeError> {
    const URL: &str = "https://gist.githubusercontent.com/ganchoradkov/\
        85f747268696d2b7585292b0b40f9d43/raw/85de5890258d08dcc5e3f4f078106883f62d43b2/index.js";

    let resp = reqwest::get(URL).await?;
    match resp.status() {
        StatusCode::OK => {}
        other => return Err(DownloadIframeError::UnexpectedStatusCode(other)),
    };

    Ok(resp.text().await?)
}

#[derive(Debug, thiserror::Error)]
enum DownloadIframeError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(StatusCode),
}
