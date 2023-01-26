use {
    crate::state::AppState,
    axum::{
        extract::{Path, State},
        http::StatusCode,
        response::{Html, IntoResponse},
    },
    std::sync::Arc,
};

pub async fn handler(
    Path(project_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html("hello world")
}
