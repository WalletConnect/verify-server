use {
    crate::{state::AppState, AttestationStore as _, Infra},
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::StatusCode,
        response::IntoResponse,
    },
    hyper::header,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationBody {
    pub attestation_id: String,
    pub origin: String,
}

pub async fn get(
    Path(attestation_id): Path<String>,
    StateExtractor(state): StateExtractor<Arc<AppState<impl Infra>>>,
) -> impl IntoResponse {
    let attestation = state
        .attestation_store()
        .get_attestation(&attestation_id)
        .await
        .unwrap();
    let resp = AttestationBody {
        origin: attestation,
        attestation_id: attestation_id.clone(),
    };

    ([(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], Json(resp))
}

pub async fn post(
    StateExtractor(state): StateExtractor<Arc<AppState<impl Infra>>>,
    body: Json<AttestationBody>,
) -> impl IntoResponse {
    let attestation_id = &body.attestation_id;
    let origin = &body.origin;
    state
        .attestation_store()
        .set_attestation(attestation_id, origin)
        .await
        .unwrap();
    (StatusCode::OK, "OK".to_string())
}
