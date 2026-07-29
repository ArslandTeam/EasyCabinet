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
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum_extra::extract::{SignedCookieJar, cookie};
use jwt_simple::{claims::Claims, prelude::*};
use sea_orm::DatabaseConnection;

pub struct AuthService;

impl AuthService {
    pub async fn authentication(
        db: &DatabaseConnection,
        cache: &CacheManager,
        login: String,
        password: &str,
        user_agent: &str,
    ) -> Result<(String, String), BackendError> {
        let user = Self::verify_auth(db, &login, password).await?;
        let session_id = uuid::Uuid::new_v4().to_string();
        Self::generate_tokens_pair(cache, &user.uuid, &user.login, &session_id, user_agent)
            .await
            .map_err(|_| BackendError::InternalError)
    }

    pub async fn register(
        db: &DatabaseConnection,
        cache: &CacheManager,
        data: auth::dto::RequestRegisterDTO,
    ) -> Result<(), BackendError> {
        let code = format!("verify_code_email:{}", data.email);

        if cache.get(&code).await? != Some(data.code.to_string()) {
            return Err(BackendError::BadRequest(
                "Invalid or expired email code".into(),
            ));
        }

        cache.delete(&code).await?;

        let hash_password = Self::generate_hash_password(data.password).await?;

        DatabaseService::create_user(db, data.login, hash_password, data.email).await?;

        Ok(())
    }

    pub async fn verify_email(
        db: &DatabaseConnection,
        cache: &CacheManager,
        email: &str,
    ) -> Result<(), BackendError> {
        if DatabaseService::find_user(db, database::entities::users::Column::Email, email)
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
        user_agent: &str,
    ) -> Result<(String, String), BackendError> {
        let payload = Self::check_token(cache, refresh_token).await?;
        cache
            .delete(&format!("session:{}:{}", payload.uuid, payload.session_id))
            .await?;
        let session_id = uuid::Uuid::new_v4().to_string();
        Self::generate_tokens_pair(
            cache,
            &payload.uuid,
            &payload.login,
            &session_id,
            user_agent,
        )
        .await
    }

    pub async fn logout(
        cache: &CacheManager,
        refresh_token: Option<String>,
    ) -> Result<(), BackendError> {
        if let Some(refresh_token) = refresh_token
            && let Ok(payload) = Self::check_token(cache, refresh_token).await
        {
            cache
                .delete(&format!("session:{}:{}", payload.uuid, payload.session_id))
                .await?;
        }

        Ok(())
    }

    pub async fn logout_all(cache: &CacheManager, uuid: String) -> Result<(), BackendError> {
        cache.delete_pattern(&format!("session:{uuid}:*")).await
    }

    pub async fn reset_password(
        db: &DatabaseConnection,
        cache: &CacheManager,
        email: &str,
    ) -> Result<(), BackendError> {
        DatabaseService::find_user(db, database::entities::users::Column::Email, email)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        let reset_token = uuid::Uuid::new_v4().to_string();

        cache
            .set(&format!("reset_token:{reset_token}"), email, 1800)
            .await?;

        email::service::send_reset_password_email(email, &reset_token).await?;

        Ok(())
    }

    // TODO перемновать
    pub async fn change_password(
        db: &DatabaseConnection,
        cache: &CacheManager,
        reset_token: String,
        password: String,
    ) -> Result<(), BackendError> {
        let email = cache
            .get(&format!("reset_token:{reset_token}"))
            .await?
            .ok_or(BackendError::BadRequest(
                "Invalid or expired reset token".into(),
            ))?;

        let user = DatabaseService::find_user(db, database::entities::users::Column::Email, &email)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        DatabaseService::update_user(
            db,
            database::entities::users::Column::Password,
            &Self::generate_hash_password(password).await?,
            database::entities::users::Column::Email,
            &email,
        )
        .await
        .map_err(|_| BackendError::InternalError)?;

        cache.delete(&format!("reset_token:{reset_token}")).await?;
        cache
            .delete_pattern(&format!("session:{}:*", user.uuid))
            .await?;

        Ok(())
    }

    pub async fn verify_auth(
        db: &DatabaseConnection,
        login: &str,
        password: &str,
    ) -> Result<database::entities::users::Model, BackendError> {
        let user = DatabaseService::find_user(db, database::entities::users::Column::Login, login)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("Invalid login or password".into()))?;

        if Self::check_password(password, &user.password).await {
            Ok(user)
        } else {
            Err(BackendError::Unauthorized(
                "Invalid login or password".into(),
            ))
        }
    }

    async fn generate_tokens_pair(
        cache: &CacheManager,
        uuid: &str,
        login: &str,
        session_id: &str,
        user_agent: &str,
    ) -> Result<(String, String), BackendError> {
        let access_token =
            Self::create_jwt_token(uuid, login, session_id, CONFIG.jwt_expires_in).await?;
        let refresh_token =
            Self::create_refresh_token(cache, session_id, login, uuid, user_agent).await?;

        Ok((access_token, refresh_token))
    }

    async fn create_jwt_token(
        uuid: &str,
        login: &str,
        session_id: &str,
        exp_range: u64,
    ) -> Result<String, BackendError> {
        let claim = auth::jwt::JwtPayload {
            uuid: uuid.to_string(),
            login: login.to_string(),
            session_id: session_id.to_string(),
        };

        let key = &CONFIG.jwt_secret;
        let claims = Claims::with_custom_claims(claim, Duration::from_secs(exp_range));
        let token = key.authenticate(claims).unwrap();

        Ok(token)
    }

    async fn create_refresh_token(
        cache: &CacheManager,
        session_id: &str,
        login: &str,
        uuid: &str,
        user_agent: &str,
    ) -> Result<String, BackendError> {
        let refresh_token =
            Self::create_jwt_token(uuid, login, session_id, CONFIG.cookie_expires_in).await?;
        cache
            .set(
                &format!("session:{uuid}:{session_id}"),
                user_agent,
                CONFIG.cookie_expires_in,
            )
            .await?;

        Ok(refresh_token)
    }

    pub async fn set_refresh_token_cookie(
        jar: SignedCookieJar,
        refresh_token: String,
    ) -> SignedCookieJar {
        let cookie = cookie::Cookie::build(("refresh_token", refresh_token))
            .path("/auth")
            .http_only(true)
            .domain(&CONFIG.cookie_domain)
            .same_site(cookie::SameSite::Lax)
            .secure(CONFIG.cookie_secure)
            .max_age(time::Duration::seconds(
                CONFIG.cookie_expires_in.try_into().unwrap(),
            ))
            .build();

        jar.add(cookie)
    }

    async fn check_token(
        cache: &CacheManager,
        token: String,
    ) -> Result<auth::jwt::JwtPayload, BackendError> {
        let key = &CONFIG.jwt_secret;
        let token_data = key
            .verify_token::<auth::jwt::JwtPayload>(&token, None)
            .map_err(|_| BackendError::Unauthorized("Invalid refresh token".into()))?;

        let claims = token_data.custom;

        if cache
            .get(&format!("session:{}:{}", claims.uuid, claims.session_id))
            .await?
            .is_none()
        {
            return Err(BackendError::Unauthorized("Session revoked".into()));
        }

        Ok(claims)
    }

    pub async fn generate_hash_password(password: String) -> Result<String, BackendError> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            argon2
                .hash_password(password.as_bytes(), &salt)
                .map(|hash| hash.to_string())
                .map_err(|e| {
                    tracing::error!("{e}");
                    BackendError::InternalError
                })
        })
        .await
        .map_err(|_| BackendError::InternalError)?
    }

    async fn check_password(password: &str, hash: &str) -> bool {
        let password = password.to_string();
        let hash = hash.to_string();
        tokio::task::spawn_blocking(move || {
            let Ok(parsed_hash) = PasswordHash::new(&hash) else {
                return false;
            };
            let argon2 = Argon2::default();

            argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        })
        .await
        .unwrap_or(false)
    }
}
