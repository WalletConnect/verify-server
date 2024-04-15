use {
    super::{Command, RequestInfo, State},
    crate::{
        GetAttestation,
        GetAttestationResult,
        Handle,
        IsScam,
        SetAttestation,
        SetAttestationResult,
    },
    axum::{
        extract::{Json, Path},
        http::StatusCode,
        response::IntoResponse,
    },
    hyper::{header, HeaderMap},
    serde::{Deserialize, Serialize},
    tracing::instrument,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Body {
    attestation_id: String,
    origin: String,
    is_scam: Option<bool>,
}

#[instrument(level = "debug", skip(s))]
pub(super) async fn get<S, G>(
    s: State<S, G>,
    Path(attestation_id): Path<String>,
    request_info: RequestInfo,
) -> Result<impl IntoResponse, StatusCode>
where
    S: for<'a> Handle<Command<GetAttestation<'a>>, Result = GetAttestationResult>,
{
    let cmd = GetAttestation {
        id: &attestation_id,
    };

    s.handle(cmd, request_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
        .map(|a| Body {
            attestation_id,
            origin: a.origin,
            is_scam: match a.is_scam {
                IsScam::Yes => Some(true),
                IsScam::No => Some(false),
                IsScam::Unknown => None,
            },
        })
        .map(|body| ([(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], Json(body)))
}

#[instrument(level = "debug", skip(s))]
pub(super) async fn post<S, G>(
    s: State<S, G>,
    headers: HeaderMap,
    request_info: RequestInfo,
    body: Json<Body>,
) -> Result<impl IntoResponse, StatusCode>
where
    S: for<'a> Handle<Command<SetAttestation<'a>>, Result = SetAttestationResult>,
{
    s.token_manager.validate_csrf_token(&headers)?;

    let cmd = SetAttestation {
        id: &body.attestation_id,
        origin: &body.origin,
    };

    s.handle(cmd, request_info)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|_| (StatusCode::OK, "OK".to_string()))
}
