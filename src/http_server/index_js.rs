use {
    axum::{
        extract::Query,
        response::{Html, IntoResponse},
    },
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
// auto reload to refresh token
window.addEventListener("load", async () => {
    setInterval(() => {
        window.location.reload()
    }, 60_000)
})
"#;

#[derive(Deserialize)]
pub(super) struct Params {
    token: String,
}

pub(super) async fn get(query: Query<Params>) -> impl IntoResponse {
    Html(TEMPLATE.replacen("{token}", &query.token, 1))
}
