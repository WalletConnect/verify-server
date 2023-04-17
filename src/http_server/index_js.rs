use axum::response::{Html, IntoResponse};

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

pub async fn get() -> impl IntoResponse {
    Html(INDEX_JS)
}
