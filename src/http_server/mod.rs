use {
    crate::{Bouncer, Domain, GetAllowedDomainsError},
    axum::{
        extract::{Path, State},
        response::{Html, IntoResponse, Response},
        routing::{get, post},
        Router,
    },
    axum_prometheus::{EndpointLabel, PrometheusMetricLayerBuilder as MetricLayerBuilder},
    futures::FutureExt,
    hyper::{header, StatusCode},
    std::{future::Future, iter, net::SocketAddr, sync::Arc},
    tap::{Pipe, Tap},
    tracing::{info, instrument},
};

mod attestation;
mod health;
mod index_js;
mod metrics;

pub async fn run(
    app: impl Bouncer,
    port: u16,
    metrics_provider: impl Fn() -> String + Clone + Send + 'static,
    metrics_port: u16,
    health_provider: impl Fn() -> String + Clone + Send + 'static,
    shutdown: impl Future,
) {
    let shutdown = shutdown
        .map(|_| info!("Shutting down servers gracefully"))
        .shared();

    let metrics_layer = MetricLayerBuilder::new()
        // We overwrite enexpected enpoint paths here, otherwise this label will collect a bunch 
        // of junk like "/+CSCOE+/logon.html".
        .with_endpoint_label_type(EndpointLabel::MatchedPathWithFallbackFn(|_| String::new()))
        .build();

    let server = Router::new()
        .route("/health", get(health::get(health_provider)))
        .route("/attestation/:attestation_id", get(attestation::get))
        .route("/attestation", post(attestation::post))
        .route("/index.js", get(index_js::get))
        .route("/:project_id", get(root))
        .layer(metrics_layer)
        .with_state(Arc::new(app))
        .into_make_service()
        .pipe(|svc| axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port))).serve(svc))
        .pipe(|s| s.with_graceful_shutdown(shutdown.clone()))
        .tap(|_| info!("Serving at :{port}"));

    let metrics_server = Router::new()
        .route("/metrics", get(metrics::get(metrics_provider)))
        .into_make_service()
        .pipe(|svc| axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], metrics_port))).serve(svc))
        .pipe(|s| s.with_graceful_shutdown(shutdown))
        .tap(|_| info!("Serving metrics at :{metrics_port}"));

    let _ = futures::join!(
        server.map(|_| info!("Server terminated")),
        metrics_server.map(|_| info!("Metrics server terminated"))
    );
}

const INDEX_HTML: &str = r#"
<!-- index.html -->
<html>
  <head>
      <script src="/index.js"></script>
  </head>
</html>
"#;

const UNKNOWN_PROJECT_MSG: &str = "Project with the provided ID doesn't exist. Please, ensure \
                                   that the project is registered on cloud.walletconnect.com";

const NO_VERIFIED_DOMAINS_MSG: &str = "Project with the provided ID doesn't have a verified \
                                       domain. Please, verify your domain on \
                                       cloud.walletconnect.com";

#[instrument(level = "debug", skip(app))]
pub async fn root(
    State(app): State<Arc<impl Bouncer>>,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let domains = app.get_allowed_domains(&project_id).await?;
    if domains.is_empty() {
        return Err((StatusCode::NOT_FOUND, NO_VERIFIED_DOMAINS_MSG).into_response());
    }

    let headers = [(
        header::CONTENT_SECURITY_POLICY,
        build_content_security_header(domains),
    )];

    Ok((headers, Html(INDEX_HTML)))
}

impl From<GetAllowedDomainsError> for Response {
    fn from(e: GetAllowedDomainsError) -> Self {
        match e {
            GetAllowedDomainsError::UnknownProject => {
                (StatusCode::NOT_FOUND, UNKNOWN_PROJECT_MSG).into_response()
            }
            GetAllowedDomainsError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

fn build_content_security_header(domains: Vec<Domain>) -> String {
    let urls = domains.iter().map(AsRef::as_ref).flat_map(|domain| {
        let proto = if domain == "localhost" {
            "http://"
        } else {
            "https://"
        };

        // `*.domain` doesn't match `domain` by the Content-Security-Policy spec, so
        // we are specifying both.
        // See the test for this function if you have any doubts.
        [" ", proto, "*.", domain, " ", proto, domain]
    });

    iter::once("frame-ancestors").chain(urls).collect()
}

#[test]
fn test_build_content_security_header() {
    fn case(domains: &[&str], expected: &str) {
        let domains = domains.into_iter().map(|s| Domain::from(s.to_string()));
        let got = build_content_security_header(domains.collect());
        assert_eq!(&got, expected);
    }

    case(
        &["walletconnect.com"],
        "frame-ancestors https://*.walletconnect.com https://walletconnect.com",
    );

    case(
        &["walletconnect.com", "vercel.app", "localhost"],
        "frame-ancestors https://*.walletconnect.com https://walletconnect.com \
                         https://*.vercel.app https://vercel.app \
                         http://*.localhost http://localhost",
    );
}
