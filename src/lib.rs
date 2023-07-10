pub use {
    anyhow::Error,
    async_trait::async_trait,
    attestation_store::AttestationStore,
    project_registry::ProjectRegistry,
};
use {
    arrayvec::ArrayString,
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
    async fn get_verify_status(
        &self,
        project_id: ProjectId,
    ) -> Result<VerifyStatus, GetVerifyStatusError>;

    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error>;
    async fn get_attestation(&self, id: &str) -> Result<Option<String>, Error>;
}

/// Status of the Verify API of some project.
pub enum VerifyStatus {
    /// Verify API is disabled.
    Disabled,

    /// Verify API is enabled.
    Enabled {
        /// List of the verified domains of the project.
        verified_domains: Vec<Domain>,
    },
}

/// Error of getting [`Verification`] via [`Bouncer::verification`].
#[derive(Debug, thiserror::Error)]
pub enum GetVerifyStatusError {
    #[error("UnknownProject")]
    UnknownProject,

    #[error(transparent)]
    Other(#[from] Error),
}

#[derive(AsRef, Clone, Debug, From, Serialize, Deserialize)]
pub struct Domain(String);

#[derive(AsRef, Clone, Copy, Debug)]
#[as_ref(forward)]
pub struct ProjectId(ArrayString<32>);

impl<'de> Deserialize<'de> for ProjectId {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        ArrayString::<32>::deserialize(de)
            .ok()
            .filter(|s| s.len() == 32 && !s.chars().any(|c| !c.is_ascii_hexdigit()))
            .map(Self)
            .ok_or(D::Error::custom(
                "ProjectId should be a hex string 32 chars long",
            ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub is_verify_enabled: bool,
    pub verified_domains: Vec<Domain>,
}

struct App<I> {
    infra: I,
}

pub fn new(infra: impl Infra) -> impl Bouncer {
    App { infra }
}

#[async_trait]
impl<I: Infra> Bouncer for App<I> {
    #[instrument(level = "warn", skip(self))]
    async fn get_verify_status(
        &self,
        project_id: ProjectId,
    ) -> Result<VerifyStatus, GetVerifyStatusError> {
        let project_data = self
            .project_registry()
            .project_data(project_id)
            .await
            .tap_err(|e| error!("ProjectRegistry::project_data: {e:?}"))?
            .ok_or(GetVerifyStatusError::UnknownProject)
            .tap_err(|_| warn!("Unknown project id"))?;

        let status = (project_data.is_verify_enabled && !project_data.verified_domains.is_empty())
            .then_some(project_data.verified_domains)
            .map(|verified_domains| VerifyStatus::Enabled { verified_domains })
            .unwrap_or(VerifyStatus::Disabled);

        Ok(status)
    }

    #[instrument(level = "debug", skip(self))]
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<(), Error> {
        self.attestation_store()
            .set_attestation(id, origin)
            .await
            .tap_err(|e| error!("AttestationStore::set_attestation: {e:?}"))
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
