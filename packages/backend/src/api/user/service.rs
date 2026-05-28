// TODO переписать
use crate::{
    BackendError,
    api::{
        assets::service::{AssetType, AssetsService},
        auth,
        cache_manager::CacheManager,
        database::{entities::users, service::DatabaseService},
        storage_manager::StorageService,
        user::dto,
    },
};
use migration::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Value};

pub struct UserService;

impl UserService {
    pub async fn get_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        cache: &CacheManager,
        uuid: String,
    ) -> Result<dto::ResponseProfileDTO, BackendError> {
        let user = DatabaseService::find_user(db, users::Column::Uuid, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;
        let textures = Self::get_textures_data(storage, &user).await;

        let sessions = cache
            .get_pattern(&format!("session:{}:*", user.uuid))
            .await?;

        Ok(dto::ResponseProfileDTO {
            login: user.login,
            textures,
            sessions,
        })
    }

    pub async fn get_textures_data(
        storage: &StorageService,
        user: &users::Model,
    ) -> dto::ResponseTexturesDTO {
        let skin_url = user
            .skin_hash
            .as_ref()
            .map(|hash| storage.format_url("skin", hash));
        let cape_url = user
            .cape_hash
            .as_ref()
            .map(|hash| storage.format_url("cape", hash));

        dto::ResponseTexturesDTO {
            is_alex: user.is_alex,
            skin_url,
            cape_url,
        }
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        user: auth::jwt::JwtPayload,
        profile: dto::RequestProfileDTO,
        skin: Option<&[u8]>,
        cape: Option<&[u8]>,
    ) -> Result<(), BackendError> {
        let mut update_user =
            users::Entity::update_many().filter(users::Column::Uuid.eq(&user.uuid));

        let user = DatabaseService::find_user(db, users::Column::Uuid, &user.uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        match (skin, profile.del_skin) {
            (Some(data), _) => {
                let hash = AssetsService::upload_image(storage, AssetType::Skin, data).await?;
                update_user = update_user
                    .col_expr(users::Column::SkinHash, Expr::value(hash))
                    .col_expr(users::Column::IsAlex, Expr::value(profile.is_alex));
            }
            (None, true) => {
                if let Some(old_hash) = user.skin_hash {
                    AssetsService::delete_image(storage, AssetType::Skin, &old_hash).await?;
                }
                update_user = update_user
                    .col_expr(users::Column::SkinHash, Expr::expr(Value::String(None)))
                    .col_expr(users::Column::IsAlex, Expr::expr(Value::Bool(None)));
            }
            (None, false) => {}
        }

        match (cape, profile.del_cape) {
            (Some(data), _) => {
                let hash = AssetsService::upload_image(storage, AssetType::Cape, data).await?;
                update_user = update_user.col_expr(users::Column::CapeHash, Expr::value(hash));
            }
            (None, true) => {
                if let Some(old_hash) = user.cape_hash {
                    AssetsService::delete_image(storage, AssetType::Cape, &old_hash).await?;
                }
                update_user =
                    update_user.col_expr(users::Column::CapeHash, Expr::expr(Value::String(None)));
            }
            (None, false) => {}
        }

        update_user
            .exec(db)
            .await
            .map_err(|_| BackendError::InternalError)?;
        Ok(())
    }

    pub async fn change_password(
        db: &DatabaseConnection,
        uuid: &str,
        password: &str,
    ) -> Result<(), BackendError> {
        users::Entity::update_many()
            .filter(users::Column::Uuid.eq(uuid))
            .col_expr(users::Column::Password, Expr::value(password))
            .exec(db)
            .await
            .map_err(|_| BackendError::InternalError)?;
        Ok(())
    }

    pub async fn change_email(
        db: &DatabaseConnection,
        cache: &CacheManager,
        uuid: &str,
        data: dto::RequestChangeEmail,
    ) -> Result<(), BackendError> {
        let code = cache
            .get_del(&format!("verify_code_email:{}", data.email))
            .await?;

        if code != Some(data.code.to_string()) {
            return Err(BackendError::BadRequest(
                "Invalid or expired email code".into(),
            ));
        }

        users::Entity::update_many()
            .filter(users::Column::Uuid.eq(uuid))
            .col_expr(users::Column::Email, Expr::value(data.email))
            .exec(db)
            .await
            .map_err(|_| BackendError::InternalError)?;

        Ok(())
    }
}
