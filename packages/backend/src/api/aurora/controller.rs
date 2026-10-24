use super::{dto::*, service::*};
use crate::{AppState, BackendError, ValidatedJson, api::auth};
use axum::{extract::State, response::IntoResponse};

pub struct AuroraController;

impl AuroraController {
    pub async fn auth(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<auth::dto::RequestLoginDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::auth(&state.db, &state.storage, payload.login, payload.password).await
    }

    pub async fn join(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<RequestJoinDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::join(&state.db, payload).await
    }

    pub async fn has_joined(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<RequestHasJoinedDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::has_join(&state.db, &state.storage, payload).await
    }

    pub async fn profile(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<RequestProfileDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::profile(&state.db, &state.storage, payload).await
    }

    pub async fn profiles(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<RequestProfilesDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::profiles(&state.db, payload).await
    }
}
