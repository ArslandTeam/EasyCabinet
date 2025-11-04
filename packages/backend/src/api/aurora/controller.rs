use crate::{
    AppState, BackendError,
    api::{aurora, auth},
};
use axum::{Json, extract::State, response::IntoResponse};

pub async fn auth(
    State(state): State<AppState>,
    Json(payload): Json<auth::dto::RequestLoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    aurora::service::auth(&state.conn, payload.login, payload.password).await
}

pub async fn join(
    State(state): State<AppState>,
    Json(payload): Json<aurora::dto::RequestJoinDto>,
) -> Result<impl IntoResponse, BackendError> {
    aurora::service::join(&state.conn, payload).await
}

pub async fn has_joined(
    State(state): State<AppState>,
    Json(payload): Json<aurora::dto::RequestHasJoinedDto>,
) -> Result<impl IntoResponse, BackendError> {
    aurora::service::has_join(&state.conn, payload).await
}

pub async fn profile(
    State(state): State<AppState>,
    Json(payload): Json<aurora::dto::RequestProfileDTO>,
) -> Result<impl IntoResponse, BackendError> {
    aurora::service::profile(&state.conn, payload).await
}

pub async fn profiles(
    State(state): State<AppState>,
    Json(payload): Json<aurora::dto::RequestProfilesDto>,
) -> Result<impl IntoResponse, BackendError> {
    aurora::service::profiles(&state.conn, payload).await
}
