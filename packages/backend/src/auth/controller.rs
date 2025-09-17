use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    BackendError,
    auth::dto::{LoginDTO, RegisterDTO},
};

pub async fn login(Json(payload): Json<LoginDTO>) -> Result<impl IntoResponse, BackendError> {
    Ok(())
}

pub async fn register(Json(payload): Json<RegisterDTO>) -> Result<impl IntoResponse, BackendError> {
    Ok(())
}
