use crate::{
    BackendError,
    api::{
        auth,
        cache_manager::CacheManager,
        database::{self, service::DatabaseService},
        email,
    },
    generate_config::CONFIG,
};
use axum_extra::extract as cookie_manager;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::DatabaseConnection;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct AuthService;

impl AuthService {
    pub async fn authentication(
        db: &DatabaseConnection,
        cache: &CacheManager,
        login: String,
        password: String,
        user_agent: String,
    ) -> Result<(String, String), BackendError> {
        let user = Self::verify_auth(db, &login, password).await?;
        let session_id = DatabaseService::create_session(db, user.uuid.clone(), user_agent)
            .await
            .map_err(|_| BackendError::InternalError)?
            .last_insert_id;
        Self::generate_tokens_pair(cache, user.uuid, user.login, session_id)
            .await
            .map_err(|_| BackendError::InternalError)
    }

    pub async fn register(
        db: &DatabaseConnection,
        cache: &CacheManager,
        data: auth::dto::RequestRegisterDTO,
    ) -> Result<(), BackendError> {
        if cache
            .get(&format!("verify_code_email:{}", data.email))
            .await
            != Some(data.code.to_string())
        {
            return Err(BackendError::BadRequest(
                "Invalid or expired email code".into(),
            ));
        }

        cache
            .delete(&format!("verify_code_email:{}", data.email))
            .await?;

        let hash_password = Self::generate_hash_password(data.password).await;

        DatabaseService::create_user(db, data.login, hash_password, data.email)
            .await
            .map_err(|_| BackendError::BadRequest("User already exists".into()))?;

        Ok(())
    }

    pub async fn verify_email(
        db: &DatabaseConnection,
        cache: &CacheManager,
        email: String,
    ) -> Result<(), BackendError> {
        if DatabaseService::find_user(db, database::entities::users::Column::Email, &email)
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
                &format!("verify_code_email:{email}"),
                &code.to_string(),
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
        let payload = Self::check_token(cache, refresh_token).await?;

        Self::generate_tokens_pair(cache, payload.uuid, payload.login, payload.session_id).await
    }

    pub async fn logout(
        cache: &CacheManager,
        db: &DatabaseConnection,
        refresh_token: Option<String>,
    ) -> Result<(), BackendError> {
        if let Some(refresh_token) = refresh_token
            && let Ok(payload) = Self::check_token(cache, refresh_token).await
        {
            DatabaseService::delete_session(db, payload.session_id)
                .await
                .map_err(|_| BackendError::InternalError)?
        }

        Ok(())
    }

    pub async fn logout_all(
        cache: &CacheManager,
        db: &DatabaseConnection,
        uuid: String,
    ) -> Result<(), BackendError> {
        DatabaseService::delete_sessions(db, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?;

        cache.delete_pattern(&format!("session:{uuid}:*")).await?;

        Ok(())
    }

    /// Создаёт два ключа:
    /// - Ключ reset_token:`reset_token` в значении присваивается `email`
    /// - Ключ email_reset_token:`email` в значении присваивается `reset_token`
    pub async fn reset_password(
        db: &DatabaseConnection,
        cache: &CacheManager,
        email: String,
    ) -> Result<(), BackendError> {
        DatabaseService::find_user(db, database::entities::users::Column::Email, &email)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        use rand::RngCore;
        let mut bytes = [0u8; 16];
        rand::rng().fill_bytes(&mut bytes);

        let reset_token = uuid::Uuid::from_bytes(bytes).to_string();

        cache
            .set(&format!("reset_token:{reset_token}"), &email, 1800)
            .await?;

        cache
            .set(&format!("email_reset_token:{email}"), &reset_token, 1800)
            .await?;

        email::service::send_reset_password_email(email, reset_token).await?;

        Ok(())
    }

    /// Ищет в кеше reset_token:`reset_token` и из него достаёт значение `email`
    ///
    /// В последствии ищет в кеше ключ email_reset_token:`email` и значение этого ключа сверяет с `reset_token`
    pub async fn change_password(
        db: &DatabaseConnection,
        cache: &CacheManager,
        reset_token: String,
        password: String,
    ) -> Result<(), BackendError> {
        let email = cache
            .get(&format!("reset_token:{reset_token}"))
            .await
            .ok_or(BackendError::BadRequest(
                "Invalid or expired reset token".into(),
            ))?;

        let current_reset_token = cache.get(&format!("email_reset_token:{email}")).await;

        if current_reset_token != Some(reset_token.to_string()) {
            return Err(BackendError::BadRequest(
                "Invalid or expired reset token".into(),
            ));
        }

        let user = DatabaseService::find_user(db, database::entities::users::Column::Email, &email)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        DatabaseService::update_user(
            db,
            database::entities::users::Column::Password,
            &Self::generate_hash_password(password).await,
            database::entities::users::Column::Email,
            &email,
        )
        .await
        .map_err(|_| BackendError::InternalError)?;

        cache.delete(&format!("email_reset_token:{email}")).await?;
        cache.delete(&format!("reset_token:{reset_token}")).await?;
        cache
            .delete_pattern(&format!("session:{}:*", user.uuid))
            .await?;

        Ok(())
    }

    pub async fn verify_auth(
        db: &DatabaseConnection,
        login: &str,
        password: String,
    ) -> Result<database::entities::users::Model, BackendError> {
        let user = DatabaseService::find_user(db, database::entities::users::Column::Login, login)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        if Self::check_password(password, &user.password).await {
            Ok(user)
        } else {
            Err(BackendError::BadRequest("Invalid password".into()))
        }
    }

    async fn generate_tokens_pair(
        cache: &CacheManager,
        uuid: String,
        login: String,
        session_id: i32,
    ) -> Result<(String, String), BackendError> {
        let access_token =
            Self::create_jwt_token(&uuid, &login, session_id, CONFIG.jwt_expresion_in).await?;
        let refresh_token = Self::create_refresh_token(cache, session_id, &login, &uuid).await?;

        Ok((access_token, refresh_token))
    }

    async fn create_jwt_token(
        uuid: &str,
        login: &str,
        session_id: i32,
        exp_range: u64,
    ) -> Result<String, BackendError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = auth::jwt::JwtPayload {
            uuid: uuid.to_string(),
            login: login.to_string(),
            session_id,
            iat: now,
            exp: now + exp_range,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        )
        .map_err(|_| BackendError::InternalError)
    }

    async fn create_refresh_token(
        cache: &CacheManager,
        session_id: i32,
        login: &str,
        uuid: &str,
    ) -> Result<String, BackendError> {
        let refresh_token =
            Self::create_jwt_token(uuid, login, session_id, CONFIG.cookie_expresion_in).await?;
        cache
            .set(
                &format!("session:{uuid}:{session_id}"),
                "1",
                CONFIG.cookie_expresion_in,
            )
            .await?;

        Ok(refresh_token)
    }

    pub async fn set_refresh_token_cookie(
        jar: cookie_manager::SignedCookieJar,
        refresh_token: String,
    ) -> cookie_manager::SignedCookieJar {
        let cookie = cookie_manager::cookie::Cookie::build(("refresh_token", refresh_token))
            .path("/auth")
            .http_only(true)
            .domain(&CONFIG.cookie_domain)
            .same_site(cookie_manager::cookie::SameSite::Lax)
            .secure(CONFIG.cookie_secure)
            .max_age(time::Duration::seconds(
                CONFIG.cookie_expresion_in.try_into().unwrap(),
            ))
            .build();

        jar.add(cookie)
    }

    async fn check_token(
        cache: &CacheManager,
        token: String,
    ) -> Result<auth::jwt::JwtPayload, BackendError> {
        let token_data = decode::<auth::jwt::JwtPayload>(
            &token,
            &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| BackendError::BadRequest("Invalid refresh token".into()))?;

        let claims = token_data.claims;

        if cache
            .get(&format!("session:{}:{}", claims.uuid, claims.session_id))
            .await
            .is_none()
        {
            return Err(BackendError::BadRequest("Session revoked".into()));
        }

        Ok(claims)
    }

    async fn generate_hash_password(password: String) -> String {
        bcrypt::hash(password, 10).unwrap()
    }

    async fn check_password(password: String, hash: &str) -> bool {
        bcrypt::verify(password, hash).unwrap()
    }
}
