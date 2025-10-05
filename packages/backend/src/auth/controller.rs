use crate::{
    AppState, BackendError,
    auth::dto::{LoginDTO, RegisterDTO},
    auth::service,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::login(&state.conn, payload).await?;
    Ok(StatusCode::OK)
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::register(&state.conn, payload).await?;
    Ok(StatusCode::CREATED)
}
