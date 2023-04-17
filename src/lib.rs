pub mod attestation_store;
pub mod http_server;
pub mod project_registry;

pub use {
    anyhow::Error, async_trait::async_trait, attestation_store::AttestationStore,
    project_registry::ProjectRegistry,
};
use {
    derive_more::{AsRef, From},
    serde::{Deserialize, Serialize},
    std::sync::atomic::{AtomicU64, Ordering},
    tap::TapFallible,
    tracing::{error, warn},
};

#[async_trait]
pub trait Bouncer: Send + Sync + 'static {
    /// Returns a list of [`UrlMatcher`]s for the project.
    async fn get_url_matchers(
        &self,
        project_id: &str,
    ) -> Result<Vec<UrlMatcher>, GetUrlMatchersError>;

    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error>;
    async fn get_attestation(&self, id: &str) -> Result<String, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetUrlMatchersError {
    #[error("UnknownProject")]
    UnknownProject,

    #[error(transparent)]
    Other(#[from] Error),
}

/// Matcher describing on which URLs a project is allowed to be served.
#[derive(Clone, Debug)]
pub struct UrlMatcher {
    /// Matching URL should have this exact [`Protocol`].
    pub protocol: Protocol,

    /// When `Some` the matching URL should have this exact
    /// [`SecondLevelDomain`]. When `None` the matching URL may have any
    /// [`SecondLevelDomain`] or none.
    pub sld: Option<SecondLevelDomain>,

    /// Matching URL should have this exact [`TopLevelDomain`].
    pub tld: TopLevelDomain,
}

impl UrlMatcher {
    pub fn localhost(protocol: Protocol) -> Self {
        Self {
            protocol,
            sld: None,
            tld: TopLevelDomain::localhost(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Protocol {
    Http,
    Https,
}

#[derive(AsRef, Clone, Debug, From, Serialize, Deserialize)]
#[as_ref(forward)]
pub struct SecondLevelDomain(String);

#[derive(AsRef, Clone, Debug, From, Serialize, Deserialize)]
#[as_ref(forward)]
pub struct TopLevelDomain(String);

impl TopLevelDomain {
    const LOCALHOST: &str = "localhost";

    pub fn localhost() -> Self {
        Self(Self::LOCALHOST.into())
    }

    pub fn is_localhost(&self) -> bool {
        self.0 == Self::LOCALHOST
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub verified_domains: Vec<(SecondLevelDomain, TopLevelDomain)>,
}

struct App<I> {
    url_whitelist: Vec<UrlMatcher>,
    infra: I,
}

pub fn new(infra: impl Infra, url_whitelist: Vec<UrlMatcher>) -> impl Bouncer {
    App {
        url_whitelist,
        infra,
    }
}

#[async_trait]
impl<I: Infra> Bouncer for App<I> {
    async fn get_url_matchers(
        &self,
        project_id: &str,
    ) -> Result<Vec<UrlMatcher>, GetUrlMatchersError> {
        let data = self
            .project_registry()
            .project_data(project_id)
            .await
            .tap_err(|e| error!("ProjectRegistry::project_data: {e:?}"))?
            .ok_or(GetUrlMatchersError::UnknownProject)
            .tap_err(|_| warn!("Unknown project id"))?;

        let matchers = data
            .verified_domains
            .into_iter()
            .map(|(sld, tld)| UrlMatcher {
                protocol: Protocol::Https,
                sld: Some(sld),
                tld,
            });

        Ok(self.url_whitelist.iter().cloned().chain(matchers).collect())
    }

    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error> {
        self.attestation_store()
            .set_attestation(id, origin)
            .await
            .tap_err(|e| error!("AttestationStore::set_attestation: {e:?}"))
    }

    async fn get_attestation(&self, id: &str) -> Result<String, Error> {
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

#[derive(Default)]
pub struct Counter(AtomicU64);

impl Counter {
    pub fn incr(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }

    pub fn u64(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}
