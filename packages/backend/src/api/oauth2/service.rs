use super::dto::*;
use crate::{
    BackendError,
    api::{
        auth::service::AuthService,
        cache_manager::CacheManager,
        database::{entities::users, service::DatabaseService},
    },
};
use http_body_util::BodyExt;
use hyper::{Response, body::Incoming};
use sea_orm::DatabaseConnection;

pub struct OAuth2Service;

impl OAuth2Service {
    pub async fn discord_token(
        res: Response<Incoming>,
    ) -> Result<DiscordTokenResponse, BackendError> {
        let body_bytes = res
            .into_body()
            .collect()
            .await
            .map_err(|e| {
                tracing::error!("Failed to read response body: {e}");
                BackendError::InternalError
            })?
            .to_bytes();

        if let Ok(text) = std::str::from_utf8(&body_bytes) {
            tracing::debug!("{}", text);
        }

        let token_data: DiscordTokenResponse =
            serde_json::from_slice(&body_bytes).map_err(|e| {
                tracing::error!("{e}");
                BackendError::InternalError
            })?;

        Ok(token_data)
    }

    pub async fn create_user_discord(
        db: &DatabaseConnection,
        cache: &CacheManager,
        username: &str,
        email: &str,
        user_agent: &str,
    ) -> Result<(String, String), BackendError> {
        let existing_user =
            DatabaseService::find::<users::Entity>(db, users::Column::Email, email).await?;

        let user = match existing_user {
            Some(user_model) => user_model,
            None => DatabaseService::create_user(db, username, None, email).await?,
        };

        let session_id = nanoid::nanoid!(15);

        Ok(
            AuthService::generate_tokens_pair(
                &cache,
                &user.uuid,
                username,
                &session_id,
                user_agent,
            )
            .await
            .map_err(|_| BackendError::InternalError)?,
        )
    }
}
