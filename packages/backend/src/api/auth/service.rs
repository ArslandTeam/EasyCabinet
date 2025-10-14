use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    BackendError,
    api::auth::{
        dto::{LoginDTO, RegisterDTO},
        jwt::{self, JwtPayload},
    },
    api::entities::users,
    api::user::service,
};
use axum_extra::extract::{
    SignedCookieJar,
    cookie::{Cookie, SameSite},
};
use bcrypt::{BcryptError, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use time::Duration;

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
    generate_tokens_pair(cache, user.uuid.unwrap(), user.login)
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
        .map_err(|_| BackendError::BadRequest("User already exists".to_string()))?;

    Ok(())
}

pub async fn refresh(
    cache: &Cache<String, String>,
    refresh_token: String,
) -> Result<(String, String), BackendError> {
    let user = check_and_remove_token(cache, refresh_token).await?;
    generate_tokens_pair(cache, user.uuid, user.login)
        .await
        .map_err(|_| BackendError::InternalError)
}

pub async fn logout(cache: &Cache<String, String>, refresh_token: String) {
    let _ = check_and_remove_token(cache, refresh_token).await;
}

fn create_access_token(uuid: String, login: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = jwt::JwtPayload {
        uuid: uuid,
        login: login,
        iat: now,
        exp: now
            + env::var("JWT_EXPIRES_IN")
                .unwrap()
                .parse::<u64>()
                .expect("JWT_EXPIRES_IN key not set in .env"),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET key not set in .env")
                .as_ref(),
        ),
    )
}

async fn create_refresh_token(cache: &Cache<String, String>, access_token: String) -> String {
    let refresh_token = uuid::Uuid::new_v4().to_string();
    cache
        .insert(format!("refreshToken:{refresh_token}"), access_token)
        .await;

    refresh_token
}

async fn generate_tokens_pair(
    cache: &Cache<String, String>,
    uuid: String,
    login: String,
) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let access_token = create_access_token(uuid, login)?;
    let refresh_token = create_refresh_token(cache, access_token.clone()).await;
    Ok((access_token, refresh_token))
}

pub fn set_refresh_token_cookie(jar: SignedCookieJar, refresh_token: String) -> SignedCookieJar {
    let cookie = Cookie::build(("refreshToken", refresh_token))
        .path("/auth")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(env::var("COOKIE_SECURE").unwrap_or_default() == "true")
        .max_age(Duration::seconds(
            env::var("COOKIE_EXPIRES_IN")
                .unwrap()
                .parse()
                .unwrap_or(2592000),
        ))
        .build();

    jar.add(cookie)
}

async fn get_refresh_token_data(
    cache: &Cache<String, String>,
    refresh_token: String,
) -> Option<String> {
    cache.get(&format!("refreshToken:{refresh_token}")).await
}

async fn add_token_to_black_list(cache: &Cache<String, String>, access_token: String) {
    cache
        .insert(format!("accessToken:{access_token}"), 1.to_string())
        .await
}

async fn delete_refresh_token(cache: &Cache<String, String>, refresh_token: String) {
    cache
        .invalidate(&format!("refreshToken:{refresh_token}"))
        .await
}

async fn check_and_remove_token(
    cache: &Cache<String, String>,
    refresh_token: String,
) -> Result<JwtPayload, BackendError> {
    let access_token = get_refresh_token_data(cache, refresh_token.clone())
        .await
        .ok_or(BackendError::BadRequest("Token not found".to_string()))?;

    let token = decode::<JwtPayload>(
        &access_token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET key not set in .env")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| BackendError::BadRequest("Invalid token".to_string()))?;

    // INFO Проверка на валидность токена
    if token.claims.exp
        > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    {
        add_token_to_black_list(cache, access_token).await;
    };

    delete_refresh_token(cache, refresh_token).await;
    Ok(JwtPayload {
        uuid: token.claims.uuid,
        login: token.claims.login,
        iat: token.claims.iat,
        exp: token.claims.exp,
    })
}

fn generate_hash_password(password: String) -> String {
    hash(password, 10).unwrap()
}

fn check_password(password: String, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
