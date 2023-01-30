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
            let resp = reqwest::get("https://gist.githubusercontent.com/pedrouid/4ceb1e95e39728ab52121128337315b3/raw/dd79085b1f67442b2cb658165753707576270a00/index.js")
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
