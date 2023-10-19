use {
    super::State,
    crate::{Bouncer, IsScam},
    axum::{
        extract::{Json, Path},
        http::StatusCode,
        response::IntoResponse,
    },
    hyper::{header, HeaderMap},
    serde::{Deserialize, Serialize},
    tracing::instrument,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Body {
    attestation_id: String,
    origin: String,
    is_scam: Option<bool>,
}

#[instrument(level = "debug", skip(s))]
pub(super) async fn get(
    s: State<impl Bouncer>,
    Path(attestation_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    s.bouncer
        .get_attestation(&attestation_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
        .map(|a| Body {
            attestation_id,
            origin: a.origin,
            is_scam: match a.is_scam {
                IsScam::Yes => Some(true),
                IsScam::No => Some(false),
                IsScam::Unknown => None,
            },
        })
        .map(|body| ([(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], Json(body)))
}

#[instrument(level = "debug", skip(s))]
pub(super) async fn post(
    s: State<impl Bouncer>,
    headers: HeaderMap,
    body: Json<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    s.token_manager.validate_csrf_token(&headers)?;

    s.bouncer
        .set_attestation(&body.attestation_id, &body.origin)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|_| (StatusCode::OK, "OK".to_string()))
}
