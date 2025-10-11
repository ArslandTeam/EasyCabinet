use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    BackendError,
    auth::{
        dto::{LoginDTO, RegisterDTO},
        jwt::{self, JwtPayload},
    },
    entities::users,
    user::service,
};
use bcrypt::{BcryptError, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

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

pub async fn login(
    db: &DatabaseConnection,
    cache: &Cache<String, String>,
    data: LoginDTO,
) -> Result<(String, String), BackendError> {
    let user = verify_auth(db, data).await?;
    let jwt_payload = jwt::JwtPayload {
        uuid: user.uuid.unwrap(),
        login: user.login,
        iat: 0,
        exp: 0,
    };
    generate_tokens_pair(cache, jwt_payload)
        .await
        .map_err(|_| BackendError::InternalError)
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
        .map_err(|_| BackendError::BadRequest(("Такой юзер есть или почта").to_string()))?;

    Ok(())
}

// TODO надо EncodingKey инициализировать в auth/jwt
fn create_access_token(user: JwtPayload) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = jwt::JwtPayload {
        uuid: user.uuid,
        login: user.login,
        iat: now,
        exp: now + 3600,
    };
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

async fn create_refresh_token(cache: &Cache<String, String>, access_token: String) -> String {
    let refresh_token = Uuid::new_v4().to_string();
    cache
        .insert(format!("refreshToken:{refresh_token}"), access_token)
        .await;

    refresh_token
}

async fn generate_tokens_pair(
    cache: &Cache<String, String>,
    user: JwtPayload,
) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let access_token = create_access_token(user)?;
    let refresh_token = create_refresh_token(cache, access_token.clone()).await;
    Ok((access_token, refresh_token))
}

fn generate_hash_password(password: String) -> String {
    hash(password, 10).unwrap()
}

fn check_password(password: String, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
