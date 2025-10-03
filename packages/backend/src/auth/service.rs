use crate::{BackendError, auth::dto::LoginDTO, entities::users, user::service};
use bcrypt::{BcryptError, hash, verify};
use sea_orm::DatabaseConnection;

pub async fn verify_auth(
    db: &DatabaseConnection,
    login: String,
    password: String,
) -> Result<users::Model, BackendError> {
    let user = service::find_user(&db, login)
        .await
        .map_err(|_| BackendError::InternalError)?;

    match user {
        Some(user) => {
            check_password(password, &user.password)
                .map_err(|_| BackendError::BadRequest("Invalid password".to_string()))?;
            Ok(user)
        }
        None => Err(BackendError::BadRequest("User not found".to_string())),
    }
}

pub async fn login(db: &DatabaseConnection, data: LoginDTO) -> Result<(), BackendError> {
    verify_auth(db, data.login, data.password).await?;
    Ok(())
}

fn generate_hash_password(password: String) -> String {
    hash(password, 10).unwrap()
}

fn check_password(password: String, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
