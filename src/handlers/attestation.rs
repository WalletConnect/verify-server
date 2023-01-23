use {
    crate::state::AppState,
    axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
};

pub async fn get(Path(attestation_id): Path<String>, State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("OK {}", attestation_id),
    )
}

pub async fn post(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("OK"),
    )
}
