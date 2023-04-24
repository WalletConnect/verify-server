pub use {
    anyhow::Error,
    async_trait::async_trait,
    attestation_store::AttestationStore,
    project_registry::ProjectRegistry,
};
use {
    derive_more::{AsRef, From},
    serde::{Deserialize, Serialize},
    tap::TapFallible,
    tracing::{error, instrument, warn},
};

pub mod attestation_store;
pub mod http_server;
pub mod project_registry;
pub mod util;

#[async_trait]
pub trait Bouncer: Send + Sync + 'static {
    /// Returns a list of [`UrlMatcher`]s for the project.
    async fn get_allowed_domains(
        &self,
        project_id: &str,
    ) -> Result<Vec<Domain>, GetAllowedDomainsError>;

    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error>;
    async fn get_attestation(&self, id: &str) -> Result<Option<String>, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetAllowedDomainsError {
    #[error("UnknownProject")]
    UnknownProject,

    #[error(transparent)]
    Other(#[from] Error),
}

#[derive(AsRef, Clone, Debug, From, Serialize, Deserialize)]
pub struct Domain(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub verified_domains: Vec<Domain>,
}

struct App<I> {
    domain_whitelist: Vec<Domain>,
    infra: I,
}

pub fn new(domain_whitelist: Vec<Domain>, infra: impl Infra) -> impl Bouncer {
    App {
        domain_whitelist,
        infra,
    }
}

#[async_trait]
impl<I: Infra> Bouncer for App<I> {
    #[instrument(level = "warn", skip(self))]
    async fn get_allowed_domains(
        &self,
        project_id: &str,
    ) -> Result<Vec<Domain>, GetAllowedDomainsError> {
        let mut domains = self
            .project_registry()
            .project_data(project_id)
            .await
            .tap_err(|e| error!("ProjectRegistry::project_data: {e:?}"))?
            .ok_or(GetAllowedDomainsError::UnknownProject)
            .tap_err(|_| warn!("Unknown project id"))?
            .verified_domains;

        domains.extend_from_slice(&self.domain_whitelist);

        Ok(domains)
    }

<<<<<<< HEAD
    #[instrument(level = "debug", skip(self))]
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error> {
        self.attestation_store()
            .set_attestation(id, origin)
            .await
            .tap_err(|e| error!("AttestationStore::set_attestation: {e:?}"))
=======
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
>>>>>>> main
    }

    #[instrument(level = "debug", skip(self))]
    async fn get_attestation(&self, id: &str) -> Result<Option<String>, Error> {
        self.attestation_store()
            .get_attestation(id)
            .await
            .tap_err(|e| error!("AttestationStore::get_attestation: {e:?}"))
    }
}

/// Infrastucture dependencies of this service.
pub trait Infra: Send + Sync + 'static {
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

impl<I: Infra> App<I> {
    pub fn attestation_store(&self) -> &I::AttestationStore {
        self.infra.attestation_store()
    }

    pub fn project_registry(&self) -> &I::ProjectRegistry {
        self.infra.project_registry()
    }
}
