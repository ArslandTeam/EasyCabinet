// FIX старые access_token токены остаются в кеше. Нужно убрать большое количество .clone()
use crate::{
    BackendError,
    api::{auth, cache_manager::CacheManager, email, entities, user},
};
use axum_extra::extract as cookie_manager;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::DatabaseConnection;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub async fn verify_auth(
    db: &DatabaseConnection,
    login: &str,
    password: String,
) -> Result<entities::users::Model, BackendError> {
    let user = user::service::find_user(db, entities::users::Column::Login, login)
        .await
        .map_err(|_| BackendError::InternalError)?;

    match user {
        Some(user) => match check_password(password, &user.password) {
            true => Ok(user),
            false => Err(BackendError::BadRequest("Invalid password".into())),
        },
        None => Err(BackendError::BadRequest("User not found".into())),
    }
}

pub async fn authentication(
    db: &DatabaseConnection,
    cache: &CacheManager,
    login: String,
    password: String,
) -> Result<(String, String), BackendError> {
    let user = verify_auth(db, &login, password).await?;
    generate_tokens_pair(cache, user.uuid.unwrap(), user.login)
        .await
        .map_err(|_| BackendError::InternalError)
}

pub async fn register(
    db: &DatabaseConnection,
    cache: &CacheManager,
    data: auth::dto::RequestRegisterDTO,
) -> Result<(), BackendError> {
    if cache.get(format!("verify_code_email:{}", data.email)).await != Some(data.code.to_string()) {
        return Err(BackendError::BadRequest(
            "Invalid or expired email code".into(),
        ));
    }

    cache
        .delete(format!("verify_code_email:{}", data.email))
        .await?;

    let hash_password = generate_hash_password(data.password);

    // TODO надо будет болле правильно обрабатывать ошибку
    user::service::create_user(db, data.login, hash_password, data.email)
        .await
        .map_err(|_| BackendError::BadRequest("User already exists".into()))?;

    Ok(())
}

pub async fn verify_email(
    db: &DatabaseConnection,
    cache: &CacheManager,
    email: String,
) -> Result<(), BackendError> {
    if user::service::find_user(db, entities::users::Column::Email, &email)
        .await
        .map_err(|_| BackendError::InternalError)?
        .is_some()
    {
        return Err(BackendError::BadRequest("User already exists".into()));
    }

    use rand::Rng;
    let code = rand::rng().random_range(100000..=999999);
    cache
        .set(
            format!("verify_code_email:{}", email),
            code.to_string(),
            900,
        )
        .await?;
    email::service::send_verify_email(email, code).await?;

    Ok(())
}

pub async fn refresh(
    cache: &CacheManager,
    refresh_token: String,
) -> Result<(String, String), BackendError> {
    let user = check_and_remove_token(cache, refresh_token).await?;
    generate_tokens_pair(cache, user.uuid, user.login)
        .await
        .map_err(|_| BackendError::InternalError)
}

pub async fn logout(cache: &CacheManager, refresh_token: String) {
    let _ = check_and_remove_token(cache, refresh_token).await;
}

pub async fn reset_password(
    db: &DatabaseConnection,
    cache: &CacheManager,
    email: String,
) -> Result<(), BackendError> {
    use rand::RngCore;
    user::service::find_user(db, entities::users::Column::Email, &email)
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::BadRequest("User not found".into()))?;

    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    let reset_token = uuid::Uuid::from_bytes(bytes).to_string();

    cache
        .set(format!("reset_token:{reset_token}"), email.clone(), 1800)
        .await?;
    email::service::send_reset_password_email(email, reset_token).await?;

    Ok(())
}

pub async fn change_password(
    db: &DatabaseConnection,
    cache: &CacheManager,
    reset_token: String,
    password: String,
) -> Result<(), BackendError> {
    let email = cache
        .get(format!("reset_token:{reset_token}"))
        .await
        .ok_or(BackendError::BadRequest(
            "Invalid or expired reset token".into(),
        ))?;

    let hash_password = generate_hash_password(password);

    user::service::update_user(
        db,
        entities::users::Column::Password,
        hash_password,
        entities::users::Column::Email,
        email,
    )
    .await
    .map_err(|_| BackendError::InternalError)?;

    cache.delete(format!("reset_token:{reset_token}")).await?;

    Ok(())
}

fn create_access_token(uuid: String, login: String) -> Result<String, BackendError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = auth::jwt::JwtPayload {
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

async fn create_refresh_token(cache: &CacheManager, access_token: String) -> String {
    let refresh_token = uuid::Uuid::new_v4().to_string();
    let _ = cache
        .set(
            format!("refresh_token:{refresh_token}"),
            access_token,
            2592000,
        )
        .await;

    refresh_token
}

async fn generate_tokens_pair(
    cache: &CacheManager,
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
    let cookie = cookie_manager::cookie::Cookie::build(("refresh_token", refresh_token))
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

async fn get_refresh_token_data(cache: &CacheManager, refresh_token: String) -> Option<String> {
    cache.get(format!("refresh_token:{refresh_token}")).await
}

// TODO убрать или переписать
async fn add_token_to_black_list(
    cache: &CacheManager,
    access_token: String,
) -> Result<(), BackendError> {
    cache
        .set(format!("access_token:{access_token}"), 1.to_string(), 300)
        .await
        .map_err(|_| BackendError::InternalError)
}

async fn delete_refresh_token(
    cache: &CacheManager,
    refresh_token: String,
) -> Result<(), BackendError> {
    cache
        .delete(format!("refresh_token:{refresh_token}"))
        .await
        .map_err(|_| BackendError::InternalError)
}

async fn check_and_remove_token(
    cache: &CacheManager,
    refresh_token: String,
) -> Result<auth::jwt::JwtPayload, BackendError> {
    let access_token = get_refresh_token_data(cache, refresh_token.clone())
        .await
        .ok_or(BackendError::BadRequest("Token not found".into()))?;

    let mut validation = Validation::default();
    validation.validate_nbf = false;
    validation.validate_exp = false;
    validation.required_spec_claims.clear();

    let token = decode::<auth::jwt::JwtPayload>(
        &access_token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET key not set in .env")
                .as_ref(),
        ),
        &validation,
    )
    .map_err(|_| BackendError::BadRequest("Invalid token".into()))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let ttl = token.claims.exp.saturating_sub(now);

    if ttl > 0 {
        add_token_to_black_list(cache, access_token.clone()).await?;
    }

    delete_refresh_token(cache, refresh_token).await?;

    Ok(auth::jwt::JwtPayload {
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
