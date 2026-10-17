// TODO добавить ValidateJson реализацию
use super::{dto::*, service::*};
use crate::{AppState, BackendError, api::auth};
use axum::{Json, extract::State, response::IntoResponse};

pub struct AuroraController;

impl AuroraController {
    pub async fn auth(
        State(state): State<AppState>,
        Json(payload): Json<auth::dto::RequestLoginDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::auth(&state.db, &state.storage, payload.login, payload.password).await
    }

    pub async fn join(
        State(state): State<AppState>,
        Json(payload): Json<RequestJoinDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::join(&state.db, payload).await
    }

    pub async fn has_joined(
        State(state): State<AppState>,
        Json(payload): Json<RequestHasJoinedDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::has_join(&state.db, &state.storage, payload).await
    }

    pub async fn profile(
        State(state): State<AppState>,
        Json(payload): Json<RequestProfileDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::profile(&state.db, &state.storage, payload).await
    }

    pub async fn profiles(
        State(state): State<AppState>,
        Json(payload): Json<RequestProfilesDto>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuroraService::profiles(&state.db, payload).await
    }
}
