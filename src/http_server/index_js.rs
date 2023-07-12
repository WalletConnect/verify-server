use axum::response::{Html, IntoResponse};

const INDEX_JS: &str = r#"
const getCookie = (name) => {
    const value = `${document.cookie}`
    const parts = value.split(`${name}=`)
    return parts.pop().split(';').shift()
}
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
            'x-csrf-token': getCookie('sid')
        })
    })
})
// auto reload to refresh cookie
window.addEventListener("load", async () => {
    setInterval(() => {
        window.location.reload()
    }, 60_000)
})"#;

pub async fn get() -> impl IntoResponse {
    Html(INDEX_JS)
}
