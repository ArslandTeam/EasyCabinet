use crate::{BackendError, api};
use axum_extra::extract as cookie_manager;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub async fn verify_auth(
    db: &DatabaseConnection,
    data: api::auth::dto::LoginDTO,
) -> Result<api::entities::users::Model, BackendError> {
    let user = api::user::service::find_user(&db, api::entities::users::Column::Login, data.login)
        .await
        .map_err(|_| BackendError::InternalError)?;

    match user {
        Some(user) => match check_password(data.password, &user.password) {
            true => Ok(user),
            false => Err(BackendError::BadRequest("Invalid password".into())),
        },
        None => Err(BackendError::BadRequest("User not found".into())),
    }
}

pub async fn login(
    db: &DatabaseConnection,
    cache: &Cache<String, String>,
    data: api::auth::dto::LoginDTO,
) -> Result<(String, String), BackendError> {
    let user = verify_auth(db, data).await?;
    generate_tokens_pair(cache, user.uuid.unwrap(), user.login)
        .await
        .map_err(|_| BackendError::InternalError)
}

pub async fn register(
    db: &DatabaseConnection,
    data: api::auth::dto::RegisterDTO,
) -> Result<(), BackendError> {
    let hash_password = generate_hash_password(data.password);

    let user = api::auth::dto::RegisterDTO {
        login: data.login,
        email: data.email,
        password: hash_password,
    };

    api::user::service::create_user(db, user)
        .await
        .map_err(|_| BackendError::BadRequest("User already exists".into()))?;

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

pub async fn reset_password(db: &DatabaseConnection, email: String) -> Result<(), BackendError> {
    use rand::RngCore;
    api::user::service::find_user(db, api::entities::users::Column::Email, email.clone())
        .await
        .map_err(|_| BackendError::BadRequest("User not found".into()))?;

    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    let reset_token = hex::encode(bytes);

    api::user::service::update_user_reset_token(db, email.clone(), reset_token.clone())
        .await
        .map_err(|_| BackendError::InternalError)?;

    api::email::service::send_reset_password_email(email, reset_token).await?;

    Ok(())
}

pub async fn change_password(
    db: &DatabaseConnection,
    reset_token: String,
    password: String,
) -> Result<(), BackendError> {
    api::user::service::find_user(
        db,
        api::entities::users::Column::ResetToken,
        reset_token.clone(),
    )
    .await
    .map_err(|_| BackendError::BadRequest("Invalid reset token".into()))?;

    let hash_password = generate_hash_password(password);

    api::user::service::change_user_password(db, reset_token, hash_password)
        .await
        .map_err(|_| BackendError::InternalError)?;

    Ok(())
}

fn create_access_token(uuid: String, login: String) -> Result<String, BackendError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = api::auth::jwt::JwtPayload {
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
    .map_err(|_| BackendError::InternalError)
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
) -> Result<(String, String), BackendError> {
    let access_token = create_access_token(uuid, login)?;
    let refresh_token = create_refresh_token(cache, access_token.clone()).await;
    Ok((access_token, refresh_token))
}

pub fn set_refresh_token_cookie(
    jar: cookie_manager::SignedCookieJar,
    refresh_token: String,
) -> cookie_manager::SignedCookieJar {
    let cookie = cookie_manager::cookie::Cookie::build(("refreshToken", refresh_token))
        .path("/auth")
        .http_only(true)
        .same_site(cookie_manager::cookie::SameSite::Lax)
        .secure(env::var("COOKIE_SECURE").unwrap_or_default() == "true")
        .max_age(time::Duration::seconds(
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
) -> Result<api::auth::jwt::JwtPayload, BackendError> {
    let access_token = get_refresh_token_data(cache, refresh_token.clone())
        .await
        .ok_or(BackendError::BadRequest("Token not found".into()))?;

    let token = decode::<api::auth::jwt::JwtPayload>(
        &access_token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET key not set in .env")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| BackendError::BadRequest("Invalid token".into()))?;

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
    Ok(api::auth::jwt::JwtPayload {
        uuid: token.claims.uuid,
        login: token.claims.login,
        iat: token.claims.iat,
        exp: token.claims.exp,
    })
}

fn generate_hash_password(password: String) -> String {
    bcrypt::hash(password, 10).expect("Error hash bcrypt")
}

fn check_password(password: String, hash: &str) -> bool {
    bcrypt::verify(password, hash).expect("Error verify bcrypt")
}
