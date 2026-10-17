use crate::{
    BackendError,
    api::{
        auth::service::AuthService,
        cache_manager::CacheManager,
        database::{entities::users, service::DatabaseService},
    },
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

pub struct OAuth2Service;

impl OAuth2Service {
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
            .map_err(|e| BackendError::InternalError(e.into()))?;

        if matched_users.len() > 1 {
            return Err(BackendError::InternalError(
                "Found two account discord error".into(),
            ));
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

        AuthService::generate_tokens_pair(cache, &user.uuid, &user.login, &session_id, user_agent)
            .await
    }

    pub async fn get_link_discord(
        cache: &CacheManager,
        uuid: &str,
    ) -> Result<String, BackendError> {
        let csrf_token = uuid::Uuid::new_v4().to_string();
        cache.set(&csrf_token, uuid, 300).await?;
        Ok(csrf_token)
    }

    pub async fn get_auth_discord(cache: &CacheManager) -> Result<String, BackendError> {
        let csrf_token = uuid::Uuid::new_v4().to_string();
        cache.set(&csrf_token, "anonymous", 300).await?;
        Ok(csrf_token)
    }

    pub async fn link_discord(
        db: &DatabaseConnection,
        cache: &CacheManager,
        discord_id: &str,
        csrf_token: &str,
    ) -> Result<(), BackendError> {
        let uuid = cache
            .get(csrf_token)
            .await?
            .ok_or_else(|| BackendError::BadRequest("Session expired or invalid token".into()))?;

        if DatabaseService::find::<users::Entity>(db, users::Column::DiscordId, discord_id)
            .await?
            .is_some()
        {
            return Err(BackendError::BadRequest(
                "This Discord account is already linked to another user".into(),
            ));
        };

        DatabaseService::update_user(
            db,
            users::Column::DiscordId,
            discord_id,
            users::Column::Uuid,
            &uuid,
        )
        .await?;

        cache.delete(csrf_token).await?;

        Ok(())
    }
}
