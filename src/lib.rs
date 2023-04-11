pub mod attestation_store;
pub mod config;
pub mod project_registry;

mod handlers;
mod state;

use {
    crate::{
        config::Configuration,
        state::{AppState, Metrics},
    },
    anyhow::Context as _,
    axum::{
        routing::{get, post},
        Router,
    },
    opentelemetry::{
        sdk::{
            metrics::selectors,
            trace::{self, IdGenerator, Sampler},
            Resource,
        },
        util::tokio_interval_stream,
        KeyValue,
    },
    opentelemetry_otlp::{Protocol, WithExportConfig},
    std::{net::SocketAddr, sync::Arc, time::Duration},
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tracing::info,
    tracing_subscriber::fmt::format::FmtSpan,
};
pub use {attestation_store::AttestationStore, project_registry::ProjectRegistry};

build_info::build_info!(fn build_info);

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = anyhow::Error;

pub async fn bootstap(mut shutdown: broadcast::Receiver<()>, config: Configuration) -> Result<()> {
    let attestation_store = attestation_store::redis::new(config.attestation_cache_url.clone())
        .context("Failed to initialize AttestationStore")?;
    let cache = project_registry::cache::redis::new(config.project_registry_cache_url.clone())
        .context("Failed to initialize project_registry::Cache")?;

    let project_registry = project_registry::cloud::new(
        config.project_registry_url.clone(),
        &config.project_registry_auth_token,
    )
    .context("Failed to initialize ProjectRegistry")?;
    let project_registry = project_registry::with_caching(project_registry, cache);

    let mut state = AppState::new(config, (attestation_store, project_registry));

    // Telemetry
    if state.config.telemetry_enabled.unwrap_or(false) {
        let grpc_url = state
            .config
            .telemetry_grpc_url
            .clone()
            .unwrap_or_else(|| "http://localhost:4317".to_string());

        let tracing_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url.clone())
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(tracing_exporter)
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(IdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_max_events_per_span(16)
                    .with_resource(Resource::new(vec![
                        KeyValue::new("service.name", state.build_info.crate_info.name.clone()),
                        KeyValue::new(
                            "service.version",
                            state.build_info.crate_info.version.clone().to_string(),
                        ),
                    ])),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .context("Failed to install opentelemetry tracer")?;

        let metrics_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url)
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let meter_provider = opentelemetry_otlp::new_pipeline()
            .metrics(tokio::spawn, tokio_interval_stream)
            .with_exporter(metrics_exporter)
            .with_period(Duration::from_secs(3))
            .with_timeout(Duration::from_secs(10))
            .with_aggregator_selector(selectors::simple::Selector::Exact)
            .build()
            .context("Failed to build opentelemetry otlp pipeline")?;

        opentelemetry::global::set_meter_provider(meter_provider.provider());

        let meter = opentelemetry::global::meter("bouncer");
        let example_counter = meter
            .i64_up_down_counter("example")
            .with_description("This is an example counter")
            .init();

        state.set_telemetry(tracer, Metrics {
            example: example_counter,
        })
    } else if !state.config.is_test {
        // Only log to console if telemetry disabled
        tracing_subscriber::fmt()
            .with_max_level(state.config.log_level())
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let port = state.config.port;

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new();

    let app = Router::new()
        .route("/health", get(handlers::health::handler))
        .route(
            "/attestation/:attestation_id",
            get(handlers::attestation::get),
        )
        .route("/index.js", get(handlers::enclave::index_js_handler))
        .route("/:project_id", get(handlers::enclave::project_handler))
        .route("/attestation", post(handlers::attestation::post))
        .layer(global_middleware)
        .with_state(state_arc);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}

/// Infrastucture dependencies of this service.
pub trait Infra {
    type AttestationStore: AttestationStore;
    type ProjectRegistry: ProjectRegistry;

    fn attestation_store(&self) -> &Self::AttestationStore;
    fn project_registry(&self) -> &Self::ProjectRegistry;
}

impl<A, P> Infra for (A, P)
where
    A: AttestationStore,
    P: ProjectRegistry,
{
    type AttestationStore = A;
    type ProjectRegistry = P;

    fn attestation_store(&self) -> &Self::AttestationStore {
        &self.0
    }

    fn project_registry(&self) -> &Self::ProjectRegistry {
        &self.1
    }
}
