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
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

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

    // TODO переписать
    pub async fn create_user_discord(
        db: &DatabaseConnection,
        cache: &CacheManager,
        discord_id: &str,
        username: &str,
        email: &str,
        user_agent: &str,
    ) -> Result<(String, String), BackendError> {
        let matched_users = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::DiscordId.eq(discord_id))
                    .add(users::Column::Email.eq(email)),
            )
            .all(db)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                BackendError::InternalError
            })?;

        if matched_users.len() > 1 {
            tracing::error!("Found two account discord error");
            return Err(BackendError::InternalError);
        }

        let existing_user = matched_users.into_iter().next();

        let user = match existing_user {
            Some(mut user_model) => {
                if user_model.discord_id.is_none() {
                    DatabaseService::update_user(
                        db,
                        users::Column::DiscordId,
                        discord_id,
                        users::Column::Uuid,
                        &user_model.uuid,
                    )
                    .await?;
                    user_model.discord_id = Some(discord_id.to_string());
                } else if user_model.discord_id.as_deref() != Some(discord_id) {
                    return Err(BackendError::BadRequest(
                        "This email is already linked to another Discord account".into(),
                    ));
                }

                user_model
            }
            None => {
                DatabaseService::create_user(db, username, None, email, Some(discord_id)).await?
            }
        };

        let session_id = nanoid::nanoid!(15);

        Ok(AuthService::generate_tokens_pair(
            &cache,
            &user.uuid,
            &user.login,
            &session_id,
            user_agent,
        )
        .await
        .map_err(|_| BackendError::InternalError)?)
    }

    pub async fn link_discord(
        db: &DatabaseConnection,
        discord_id: &str,
        uuid: &str,
    ) -> Result<(), BackendError> {
        let conflict =
            DatabaseService::find::<users::Entity>(db, users::Column::DiscordId, discord_id)
                .await?;

        if conflict.is_some() {
            return Err(BackendError::BadRequest(
                "This Discord account is already linked to another user".into(),
            ));
        }

        DatabaseService::update_user(
            &db,
            users::Column::DiscordId,
            discord_id,
            users::Column::Uuid,
            uuid,
        )
        .await?;

        Ok(())
    }
}
