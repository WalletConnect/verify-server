use {
    super::CsrfToken,
    axum::{
        extract::Query,
        response::{Html, IntoResponse},
    },
    hyper::StatusCode,
    serde::Deserialize,
};

const TEMPLATE: &str = r#"
const csrfToken = '{token}';
// event subscribed by Verify Enclave
window.addEventListener("message", (event) => {
    const attestationId = event.data
    const origin = event.origin
    if (!attestationId) return
    if (attestationId.length !== 64) return
    fetch(`${window.location.protocol}//${window.location.host}/attestation`, {
        method: "POST",
        body: JSON.stringify({ attestationId, origin }),
        headers: new Headers({ 
            'content-type': 'application/json',
            'x-csrf-token': csrfToken
        })
    })
})
// token is valid for 1 hour, refresh every 55 minutes by reloading
window.addEventListener("load", async () => {
    setInterval(() => {
        window.location.reload()
    }, 1000 * 60 * 55)
})

// notify the SDK that the iframe is ready
window.parent.postMessage("verify_ready", "*")
"#;

#[derive(Deserialize)]
pub(super) struct Params {
    token: String,
}

pub(super) async fn get(query: Query<Params>) -> Result<impl IntoResponse, StatusCode> {
    if !CsrfToken::validate_format(&query.token) {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Html(TEMPLATE.replacen("{token}", &query.token, 1)))
}
