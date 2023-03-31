use {
    crate::state::AppState,
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
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let body = match path.as_str() {
        "index.js" => download_iframe_js()
            .await
            .map_err(|e| log::error!("Failed to download iframe: {}", e))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .map(Html)?,
        _ => Html(INDEX_HTML.into()),
    };

    let policy = "frame-ancestors https://react-wallet.walletconnect.com";

    Ok(([(header::CONTENT_SECURITY_POLICY, policy)], body))
}

// TODO: bundle it / download during initialization
async fn download_iframe_js() -> Result<String, reqwest::Error> {
    const URL: &str = "https://ist.githubusercontent.com/ganchoradkov/\
        85f747268696d2b7585292b0b40f9d43/raw/85de5890258d08dcc5e3f4f078106883f62d43b2/index.js";

    reqwest::get(URL).await?.text().await
}
