use crate::{
    AppState, BackendError,
    auth::dto::{LoginDTO, RegisterDTO},
    auth::service,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    let (access_token, refresh_token) = service::login(&state.conn, &state.cache, payload).await?;
    Ok((StatusCode::OK, Json(json!({"accessToken": access_token}))))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::register(&state.conn, payload).await?;
    Ok(StatusCode::CREATED)
}
