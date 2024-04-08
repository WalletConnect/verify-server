use {
    anyhow::Context as _,
    arrayvec::ArrayString,
    derive_more::{AsRef, From},
    serde::{Deserialize, Serialize},
    std::time::Duration,
    tap::{Tap, TapFallible, TapOptional},
    tracing::{error, instrument, warn},
    wc::future::FutureExt as _,
};
pub use {
    anyhow::Error, async_trait::async_trait, attestation_store::AttestationStore,
    event_sink::EventSink, project_registry::ProjectRegistry, scam_guard::ScamGuard,
};

pub mod attestation_store;
pub mod cache;
pub mod event_sink;
pub mod http_server;
pub mod project_registry;
pub mod scam_guard;
pub mod util;

#[async_trait]
pub trait Handle<Cmd>: Send + Sync + 'static {
    type Result;

    async fn handle(&self, cmd: Cmd) -> Self::Result
    where
        Cmd: 'async_trait;
}

#[derive(Debug, Clone, Copy)]
pub struct GetVerifyStatus<'a> {
    pub project_id: &'a ProjectId,
}

/// Status of the Verify API of some project.
#[derive(Debug)]
pub enum VerifyStatus {
    /// Verify API is disabled.
    Disabled,

    /// Verify API is enabled.
    Enabled {
        /// List of the verified domains of the project.
        verified_domains: Vec<Domain>,
    },
}

/// Error of getting a [`VerifyStatus`] via [`Bouncer::get_verify_status`].
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

pub type GetVerifyStatusResult = Result<VerifyStatus, GetVerifyStatusError>;

#[async_trait]
impl<'a, I: Infra> Handle<GetVerifyStatus<'a>> for Service<I> {
    type Result = GetVerifyStatusResult;

    #[instrument(level = "warn", skip(self))]
    async fn handle(&self, cmd: GetVerifyStatus<'a>) -> Self::Result {
        let project_data = self
            .project_registry()
            .project_data(cmd.project_id)
            .with_timeout(Duration::from_secs(10))
            .await
            .context("ProjectRegistry::project_data timed out")?
            .tap_err(|e| error!("ProjectRegistry::project_data: {e:?}"))?
            .ok_or(GetVerifyStatusError::UnknownProject)
            .tap_err(|_| warn!("Unknown project id"))?;

        let status = (project_data.is_verify_enabled && !project_data.verified_domains.is_empty())
            .then_some(project_data.verified_domains)
            .map(|verified_domains| VerifyStatus::Enabled { verified_domains })
            .unwrap_or(VerifyStatus::Disabled);

        Ok(status)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetAttestation<'a> {
    pub id: &'a str,
    pub origin: &'a str,
}

#[derive(Debug)]
pub struct Attestation {
    /// The origin domain of this attestation.
    pub origin: String,

    /// Indicator of whether the [`Attestation::origin`] domain represents a
    /// scam dApp or not.
    pub is_scam: IsScam,
}

/// Indicator of whether a domain represents a scam dApp or not.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum IsScam {
    Yes,
    No,
    Unknown,
}

pub type SetAttestationResult = anyhow::Result<()>;

#[async_trait]
impl<'a, I: Infra> Handle<SetAttestation<'a>> for Service<I> {
    type Result = SetAttestationResult;

    #[instrument(level = "debug", skip(self))]
    async fn handle(&self, cmd: SetAttestation<'a>) -> Self::Result {
        self.attestation_store()
            .set_attestation(cmd.id, cmd.origin)
            .await
            .tap_err(|e| error!("AttestationStore::set_attestation: {e:?}"))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetAttestation<'a> {
    pub id: &'a str,
}

pub type GetAttestationResult = anyhow::Result<Option<Attestation>>;

#[async_trait]
impl<'a, I: Infra> Handle<GetAttestation<'a>> for Service<I> {
    type Result = GetAttestationResult;

    #[instrument(level = "debug", skip(self))]
    async fn handle(&self, cmd: GetAttestation<'a>) -> Self::Result {
        let origin = self
            .attestation_store()
            .get_attestation(cmd.id)
            .await
            .tap_err(|e| error!("AttestationStore::get_attestation: {e:?}"))?;

        let Some(origin) = origin else {
            return Ok(None);
        };

        let is_scam = self
            .scam_guard()
            .is_scam(&origin)
            .with_timeout(Duration::from_secs(10))
            .await
            .map_err(|_| error!("ScamGuard::is_scam timed out"))
            .ok()
            .and_then(|res| res.map_err(|e| error!("ScamGuard::is_scam: {e:?}")).ok())
            .unwrap_or(IsScam::Unknown);

        Ok(Some(Attestation { origin, is_scam }))
    }
}

pub struct Service<I> {
    infra: I,
}

impl<I> Service<I> {
    pub fn new(infra: I) -> Service<I> {
        Service { infra }
    }

    pub fn observable<E>(self, event_sink: Option<E>) -> Observable<Self, E> {
        Observable {
            service: self,
            event_sink,
        }
    }
}

/// Infrastucture dependencies of this service.
pub trait Infra: Send + Sync + 'static {
    type AttestationStore: AttestationStore;
    type ProjectRegistry: ProjectRegistry;
    type ScamGuard: ScamGuard;

    fn attestation_store(&self) -> &Self::AttestationStore;
    fn project_registry(&self) -> &Self::ProjectRegistry;
    fn scam_guard(&self) -> &Self::ScamGuard;
}

impl<A, P, S> Infra for (A, P, S)
where
    A: AttestationStore,
    P: ProjectRegistry,
    S: ScamGuard,
{
    type AttestationStore = A;
    type ProjectRegistry = P;
    type ScamGuard = S;

    fn attestation_store(&self) -> &Self::AttestationStore {
        &self.0
    }

    fn project_registry(&self) -> &Self::ProjectRegistry {
        &self.1
    }

    fn scam_guard(&self) -> &Self::ScamGuard {
        &self.2
    }
}

impl<I: Infra> Service<I> {
    pub fn attestation_store(&self) -> &I::AttestationStore {
        self.infra.attestation_store()
    }

    pub fn project_registry(&self) -> &I::ProjectRegistry {
        self.infra.project_registry()
    }

    pub fn scam_guard(&self) -> &I::ScamGuard {
        self.infra.scam_guard()
    }
}

/// Command with an execution context attached to it.
#[derive(Debug)]
pub struct ContextualCommand<Cmd, Ctx> {
    pub inner: Cmd,
    pub context: Ctx,
}

/// Event of a command being handled.
#[derive(Debug)]
pub struct CommandHandled<Cmd, Ctx, Res> {
    pub cmd: ContextualCommand<Cmd, Ctx>,
    pub result: Res,
}

/// Event of [`GetVerifyStatus`] command being handled.
pub type GetVerifyStatusHandled<'c, 'r, Ctx> =
    CommandHandled<GetVerifyStatus<'c>, Ctx, &'r GetVerifyStatusResult>;

/// Event of [`SetAttestation`] command being handled.
pub type SetAttestationHandled<'c, 'r, Ctx> =
    CommandHandled<SetAttestation<'c>, Ctx, &'r SetAttestationResult>;

/// Event of [`GetAttestation`] command being handled.
pub type GetAttestationHandled<'c, 'r, Ctx> =
    CommandHandled<GetAttestation<'c>, Ctx, &'r GetAttestationResult>;

/// Observable [`Service`] emmitting [`CommandHandled`] events to an
/// [`EventSink`].
pub struct Observable<S, E> {
    service: S,
    event_sink: Option<E>,
}

#[async_trait]
impl<Cmd, Ctx, S, E> Handle<ContextualCommand<Cmd, Ctx>> for Observable<S, E>
where
    Cmd: Send + Copy,
    Ctx: Send,
    S: Handle<Cmd>,
    for<'a> E: EventSink<CommandHandled<Cmd, Ctx, &'a S::Result>>,
{
    type Result = S::Result;

    async fn handle(&self, cmd: ContextualCommand<Cmd, Ctx>) -> Self::Result
    where
        ContextualCommand<Cmd, Ctx>: 'async_trait,
    {
        self.service.handle(cmd.inner).await.tap(|result| {
            self.event_sink
                .as_ref()
                .tap_some(|sink| sink.send(CommandHandled { cmd, result }));
        })
    }
}
