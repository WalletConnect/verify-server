use {
    crate::{Configuration, Infra},
    build_info::BuildInfo,
    opentelemetry::{metrics::UpDownCounter, sdk::trace::Tracer},
    tracing_subscriber::prelude::*,
};

#[derive(Clone)]
pub struct Metrics {
    pub example: UpDownCounter<i64>,
}

#[derive(Clone)]
pub struct AppState<I> {
    pub config: Configuration,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,

    infra: I,
}

build_info::build_info!(fn build_info);

impl<I> AppState<I> {
    pub fn new(config: Configuration, infra: I) -> Self {
        let build_info: &BuildInfo = build_info();

        AppState {
            config,
            build_info: build_info.clone(),
            metrics: None,
            infra,
        }
    }

    pub fn set_telemetry(&mut self, tracer: Tracer, metrics: Metrics) {
        let otel_tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(otel_tracing_layer)
            .init();

        self.metrics = Some(metrics);
    }
}

impl<I: Infra> AppState<I> {
    pub fn attestation_store(&self) -> &I::AttestationStore {
        self.infra.attestation_store()
    }

    pub fn project_registry(&self) -> &I::ProjectRegistry {
        self.infra.project_registry()
    }
}
