use {
    crate::state::AppState,
    axum::{
        extract::{Path, State},
        response::{Html, IntoResponse},
    },
    std::sync::Arc,
};

pub async fn handler(
    Path(project_id): Path<String>,
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match project_id.as_str() {
        "index.js" => {
            let resp = reqwest::get("https://gist.githubusercontent.com/ganchoradkov/85f747268696d2b7585292b0b40f9d43/raw/7f925d14fb2d9e73f26c3362b441073de70a8121/index.js")
                .await.unwrap()
                .text()
                .await.unwrap();
            Html(resp)
        }
        _ => Html(
            r#"
        <!-- index.html -->
        <html>
          <head>
              <script src="/index.js"></script>
          </head>
        </html>
        "#
            .into(),
        ),
    }
}
