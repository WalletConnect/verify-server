use {
    crate::{Bouncer, GetUrlMatchersError, Protocol, UrlMatcher},
    axum::{
        extract::{Path, State},
        response::{Html, IntoResponse},
        routing::{get, post},
        Router,
    },
    futures::FutureExt,
    hyper::{header, StatusCode},
    std::{future::Future, iter, net::SocketAddr, sync::Arc},
    tap::{Pipe, Tap},
    tracing::{info, instrument},
};

#[rustfmt::skip]
// We don't actually depend on prometheus here, we only use it for `axum ->
// metrics` integration. See: https://github.com/Ptrskay3/axum-prometheus/issues/16
use axum_prometheus::PrometheusMetricLayer as MetricLayer;

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

    let server = Router::new()
        .route("/health", get(health::get(health_provider)))
        .route("/attestation/:attestation_id", get(attestation::get))
        .route("/attestation", post(attestation::post))
        .route("/index.js", get(index_js::get))
        .route("/:project_id", get(root))
        .layer(MetricLayer::new())
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

#[instrument(level = "debug", skip(app))]
pub async fn root(
    State(app): State<Arc<impl Bouncer>>,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let content_security = app
        .get_url_matchers(&project_id)
        .await
        .map(build_content_security_header)?;

    let headers = [(header::CONTENT_SECURITY_POLICY, content_security)];

    Ok((headers, Html(INDEX_HTML)))
}

impl From<GetUrlMatchersError> for StatusCode {
    fn from(e: GetUrlMatchersError) -> Self {
        match e {
            GetUrlMatchersError::UnknownProject => StatusCode::NOT_FOUND,
            GetUrlMatchersError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn build_content_security_header(matchers: Vec<UrlMatcher>) -> String {
    let urls = matchers.iter().flat_map(|m| {
        let proto = match m.protocol {
            Protocol::Http => "http://",
            Protocol::Https => "https://",
        };
        let tld = m.tld.as_ref();
        let (sld, sep) = m
            .sld
            .as_ref()
            .map(|sld| (sld.as_ref(), "."))
            .unwrap_or_default();

        // `*.sld.tld` doesn't match `sld.tld` by the Content-Security-Policy spec, so
        // we are specifying both `*.sld.tld` and `sld.tld`.
        // See the test for this function if you have any doubts.
        [" ", proto, "*.", sld, sep, tld, " ", proto, sld, sep, tld]
    });

    iter::once("frame-ancestors").chain(urls).collect()
}

#[test]
fn test_build_content_security_header() {
    fn case(matchers: &[(&str, Option<&str>, &str)], expected: &str) {
        let matchers = matchers.into_iter().map(|(proto, sld, tld)| UrlMatcher {
            protocol: match *proto {
                "http" => Protocol::Http,
                "https" => Protocol::Https,
                _ => unreachable!(),
            },
            sld: sld.map(|s| s.to_string().into()),
            tld: tld.to_string().into(),
        });

        let got = build_content_security_header(matchers.collect());
        assert_eq!(&got, expected);
    }

    case(
        &[("https", Some("walletconnect"), "com")],
        "frame-ancestors https://*.walletconnect.com https://walletconnect.com",
    );

    case(
        &[
            ("https", Some("walletconnect"), "com"),
            ("https", Some("vercel"), "app"),
            ("http", None, "localhost"),
        ],
        "frame-ancestors https://*.walletconnect.com https://walletconnect.com \
                         https://*.vercel.app https://vercel.app \
                         http://*.localhost http://localhost",
    );
}
