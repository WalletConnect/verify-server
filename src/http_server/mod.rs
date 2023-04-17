use std::time::Duration;

use {
    crate::{Bouncer, GetUrlMatchersError, Protocol, UrlMatcher},
    axum::{
        extract::{Path, State},
        response::{Html, IntoResponse},
        routing::{get, post},
        Router,
    },
    hyper::{header, StatusCode},
    std::{future::Future, iter, net::SocketAddr, sync::Arc},
    tokio::select,
    tower::ServiceBuilder,
    tracing::{info, instrument},
};

use ::metrics::histogram;
use axum::{extract::MatchedPath, response::Response};
use hyper::{Body, Request};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, TraceLayer};
use tracing::{debug, error, info_span, warn, Span};

mod attestation;
mod health;
mod index_js;
mod metrics;

pub async fn run(app: impl Bouncer, port: u16, health_resp: String, shutdown: impl Future) {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<Body>| {
            info_span!(
                "request",
                method = %req.method(),
                path = %req.extensions().get::<MatchedPath>().map(|p| p.as_str()).unwrap_or_default(),
            )
        })
        .on_request(DefaultOnRequest::new())
        .on_response(|resp: &Response, latency: Duration, span: &Span| {
            let method = span.field("method").map(|v| v.to_string()).unwrap_or_default();

            histogram!("latency", latency, "method" => method);
            let _span = span.enter();
            match resp.status().as_u16() {
                s @ 200..=299 => debug!("{s}"),
                s @ 400..=499 => warn!("{s}"),
                s @ 500..=599 => error!("{s}"),
                _ => {}
            };
        });

    let app = Router::new()
        .route("/health", get(health::get(health_resp)))
        .route("/attestation/:attestation_id", get(attestation::get))
        .route("/attestation", post(attestation::post))
        .route("/index.js", get(index_js::get))
        .route("/:project_id", get(root))
        .layer(trace_layer)
        .with_state(Arc::new(app));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Listening at :{port}");
    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = shutdown => info!("Shutdown signal received, killing the server"),
    }
}

const INDEX_HTML: &str = r#"
<!-- index.html -->
<html>
  <head>
      <script src="/index.js"></script>
  </head>
</html>
"#;

#[instrument(skip(app))]
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
        // we are specifying both `*.sld.tld` and `sld.tld`
        [" ", proto, "*.", sld, sep, tld, " ", proto, sld, sep, tld]
    });

    iter::once("frame-ancestors").chain(urls).collect()
}

impl From<GetUrlMatchersError> for StatusCode {
    fn from(e: GetUrlMatchersError) -> Self {
        match e {
            GetUrlMatchersError::UnknownProject => StatusCode::NOT_FOUND,
            GetUrlMatchersError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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
