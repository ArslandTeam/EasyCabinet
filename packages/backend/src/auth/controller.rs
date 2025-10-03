use crate::{
    AppState, BackendError,
    auth::dto::{LoginDTO, RegisterDTO},
    auth::service,
};
use axum::{Json, extract::State, response::IntoResponse};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::login(&state.conn, payload).await?;
    Ok(())
}

pub async fn register(Json(payload): Json<RegisterDTO>) -> Result<impl IntoResponse, BackendError> {
    Ok(())
}
