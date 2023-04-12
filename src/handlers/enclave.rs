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

pub async fn project_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState<impl Infra>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let verified_domain = state
        .project_registry()
        .project_data(&id)
        .await
        .map_err(|e| log::error!("Failed to query ProjectData: {e}"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .and_then(|data| data.verified_domain)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Project registry is expected to return domains in the form of `{name}.{TLD}`.
    // By appending `https://*.` we are allowing the iframe to be served to any
    // subdomain over HTTPS.
    let mut policy = format!("frame-ancestors https://*.{verified_domain}");

    // We allow the iframe to be served to some additional domains in dev
    // environmensts.
    if state.config.is_dev {
        policy.push_str(" https://*.walletconnect.com https://*.vercel.app *.localhost");
    }

    let headers = [(header::CONTENT_SECURITY_POLICY, policy)];

    Ok((headers, Html(INDEX_HTML)))
}

const INDEX_JS: &str = r#"
// event subscribed by Verify Enclave
window.addEventListener("message", (event) => {
    const attestationId = event.data
    const origin = event.origin
    if (!attestationId) return
    fetch(`${window.location.protocol}//${window.location.host}/attestation`, {
        method: "POST",
        body: JSON.stringify({ attestationId, origin }),
        headers: new Headers({ 'content-type': 'application/json' })
    })
})"#;

pub async fn index_js_handler() -> impl IntoResponse {
    Html(INDEX_JS)
}
