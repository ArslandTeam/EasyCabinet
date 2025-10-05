use crate::{
    BackendError,
    auth::dto::{LoginDTO, RegisterDTO},
    entities::users,
    user::service,
};
use bcrypt::{BcryptError, hash, verify};
use sea_orm::DatabaseConnection;

pub async fn verify_auth(
    db: &DatabaseConnection,
    data: LoginDTO,
) -> Result<users::Model, BackendError> {
    let user = service::find_user(&db, &data)
        .await
        .map_err(|_| BackendError::InternalError)?;

    match user {
        Some(user) => match check_password(data.password, &user.password) {
            Ok(true) => Ok(user),
            Ok(false) => Err(BackendError::BadRequest("Invalid password".to_string())),
            Err(_) => Err(BackendError::InternalError),
        },
        None => Err(BackendError::BadRequest("User not found".to_string())),
    }
}

pub async fn login(db: &DatabaseConnection, data: LoginDTO) -> Result<(), BackendError> {
    verify_auth(db, data).await?;
    Ok(())
}

pub async fn register(db: &DatabaseConnection, data: RegisterDTO) -> Result<(), BackendError> {
    let hash_password = generate_hash_password(data.password);

    let user = RegisterDTO {
        login: data.login,
        email: data.email,
        password: hash_password,
    };

    service::create_user(db, user)
        .await
        .map_err(|_| BackendError::InternalError)?;

    Ok(())
}

fn generate_hash_password(password: String) -> String {
    hash(password, 10).unwrap()
}

fn check_password(password: String, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
