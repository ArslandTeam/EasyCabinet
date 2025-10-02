use axum::response::IntoResponse;
use sea_orm::DatabaseConnection;

use crate::{BackendError, auth::dto::LoginDTO, user::service};

pub async fn verify_auth(
    db: &DatabaseConnection,
    login: String,
    password: String,
) -> Result<impl IntoResponse, BackendError> {
    let user = service::find_user(&db, login)
        .await
        .map_err(|_| BackendError::InternalError)?;

    match user {
        Some(_) => Ok(()),
        None => Err(BackendError::BadRequest("User not found".to_string())),
    }
}

pub async fn login(
    db: &DatabaseConnection,
    data: LoginDTO,
) -> Result<impl IntoResponse, BackendError> {
    verify_auth(db, data.login, data.password).await
}
