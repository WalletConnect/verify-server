use {
    crate::{Bouncer, Domain, GetVerifyStatusError, ProjectId, VerifyStatus},
    axum::{
        body::HttpBody,
        extract::Path,
        response::{Html, IntoResponse, Response},
        routing::{get, post},
        Router,
    },
    axum_prometheus::{EndpointLabel, PrometheusMetricLayerBuilder as MetricLayerBuilder},
    futures::FutureExt,
    hyper::{
        header,
        http::{HeaderName, HeaderValue},
        HeaderMap,
        Method,
        StatusCode,
    },
    serde::{Deserialize, Serialize},
    std::{future::Future, iter, net::SocketAddr, sync::Arc},
    tap::{Pipe, Tap},
    tower_http::cors::{self, CorsLayer},
    tracing::{info, instrument},
    wc::geoip::{
        block::{middleware::GeoBlockLayer, BlockingPolicy as GeoBlockingPolicy},
        MaxMindResolver,
    },
};

mod attestation;
mod health;
mod index_js;
mod metrics;

struct Server<B> {
    bouncer: B,
    encoding_key: jsonwebtoken::EncodingKey,
    decoding_key: jsonwebtoken::DecodingKey,
}

type State<B> = axum::extract::State<Arc<Server<B>>>;

pub async fn run(
    app: impl Bouncer,
    port: u16,
    secret: &[u8],
    blocked_countries: Vec<String>,
    metrics_provider: impl Fn() -> String + Clone + Send + 'static,
    metrics_port: u16,
    health_provider: impl Fn() -> String + Clone + Send + 'static,
    geoip_resolver: Option<Arc<MaxMindResolver>>,
    shutdown: impl Future,
) {
    let shutdown = shutdown
        .map(|_| info!("Shutting down servers gracefully"))
        .shared();

    let cors_layer = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods([Method::OPTIONS, Method::GET]);

    let metrics_layer = MetricLayerBuilder::new()
        // We overwrite enexpected enpoint paths here, otherwise this label will collect a bunch
        // of junk like "/+CSCOE+/logon.html".
        .with_endpoint_label_type(EndpointLabel::MatchedPathWithFallbackFn(|_| String::new()))
        .build();

    let state = Server {
        bouncer: app,
        encoding_key: jsonwebtoken::EncodingKey::from_secret(secret),
        decoding_key: jsonwebtoken::DecodingKey::from_secret(secret),
    };

    let server: Router = Router::new()
        .route("/attestation/:attestation_id", get(attestation::get))
        .layer(cors_layer)
        .route("/health", get(health::get(health_provider)))
        .route("/attestation", post(attestation::post))
        .route("/index.js", get(index_js::get))
        .route("/:project_id", get(root))
        .layer(metrics_layer)
        .with_state(Arc::new(state));
    let server = if let Some(resolver) = geoip_resolver.clone() {
        server.layer(GeoBlockLayer::new(
            resolver,
            blocked_countries.clone(),
            GeoBlockingPolicy::AllowAll,
        ))
    } else {
        server
    };
    let server = server
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

fn index_html(token: &str) -> String {
    format!(
        "<!-- index.html --><html><head><script \
         src=\"/index.js?token={token}\"></script></head></html>"
    )
}

const UNKNOWN_PROJECT_MSG: &str = "Project with the provided ID doesn't exist. Please, ensure \
                                   that the project is registered on cloud.walletconnect.com";

#[instrument(level = "debug", skip(s))]
async fn root(
    s: State<impl Bouncer>,
    Path(project_id): Path<ProjectId>,
) -> Result<Response, Response> {
    Ok(match s.bouncer.get_verify_status(project_id).await? {
        VerifyStatus::Disabled => String::new().into_response(),
        VerifyStatus::Enabled { verified_domains } => {
            let token = s.generate_csrf_token()?;
            let html = index_html(&token);
            let csp = build_content_security_header(verified_domains);
            let headers = [
                (header::CONTENT_SECURITY_POLICY, csp),
                (CsrfToken::header_name(), token),
            ];
            (headers, Html(html)).into_response()
        }
    })
}

impl From<GetVerifyStatusError> for Response {
    fn from(e: GetVerifyStatusError) -> Self {
        match e {
            GetVerifyStatusError::UnknownProject => {
                (StatusCode::NOT_FOUND, UNKNOWN_PROJECT_MSG).into_response()
            }
            GetVerifyStatusError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CsrfToken {
    exp: usize,
}

impl CsrfToken {
    // Using const value instead of a fn produces this warning:
    // https://rust-lang.github.io/rust-clippy/master/index.html#declare_interior_mutable_const
    const fn header_name() -> HeaderName {
        HeaderName::from_static("x-csrf-token")
    }

    /// Validates the format of the token without checking either signature or
    /// claims.
    fn validate_format(s: &str) -> bool {
        jsonwebtoken::decode_header(s).is_ok()
    }
}

impl<B> Server<B> {
    fn generate_csrf_token(&self) -> Result<String, Response> {
        use jsonwebtoken::{encode, get_current_timestamp, Header};

        const TTL_SECS: usize = 60 * 60; // 1 hour

        let claims = CsrfToken {
            exp: get_current_timestamp() as usize + TTL_SECS,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
    }

    fn validate_csrf_token(&self, headers: &HeaderMap<HeaderValue>) -> Result<(), StatusCode> {
        use jsonwebtoken::{decode, Validation};

        let try_validate = |headers: &HeaderMap<HeaderValue>| {
            let token = headers.get(CsrfToken::header_name())?.to_str().ok()?;

            decode::<CsrfToken>(token, &self.decoding_key, &Validation::default())
                .map(drop)
                .ok()
        };

        try_validate(headers).ok_or(StatusCode::FORBIDDEN)
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
