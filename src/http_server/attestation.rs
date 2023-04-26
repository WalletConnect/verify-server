use {
    crate::Bouncer,
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::StatusCode,
        response::IntoResponse,
    },
    hyper::header,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tracing::instrument,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationBody {
    pub attestation_id: String,
    pub origin: String,
}

#[instrument(level = "debug", skip(app))]
pub async fn get(
    StateExtractor(app): StateExtractor<Arc<impl Bouncer>>,
    Path(attestation_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    app.get_attestation(&attestation_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
        .map(|origin| AttestationBody {
            attestation_id,
            origin,
        })
        .map(|body| ([(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], Json(body)))
}

#[instrument(level = "debug", skip(app))]
pub async fn post(
    StateExtractor(app): StateExtractor<Arc<impl Bouncer>>,
    body: Json<AttestationBody>,
) -> Result<impl IntoResponse, StatusCode> {
    app.set_attestation(&body.attestation_id, &body.origin)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|_| (StatusCode::OK, "OK".to_string()))
}
