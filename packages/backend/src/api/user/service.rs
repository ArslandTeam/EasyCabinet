// TODO переписать
use crate::{
    BackendError,
    api::{
        assets, auth,
        database::{entities::users, service::DatabaseService},
        storage::service::StorageService,
        user::dto,
    },
};
use migration::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Default)]
pub struct UserService;

impl UserService {
    pub async fn get_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        uuid: String,
    ) -> Result<dto::ResponseProfileDTO, BackendError> {
        let user = DatabaseService::find_user(db, users::Column::Uuid, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        let sessions = DatabaseService::get_sessions(db, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?;

        let skin_url = user
            .skin_hash
            .as_ref()
            .map(|hash| storage.format_url("skin", hash));
        let cape_url = user
            .cape_hash
            .as_ref()
            .map(|hash| storage.format_url("cape", hash));

        Ok(dto::ResponseProfileDTO {
            is_alex: user.is_alex,
            skin_url,
            cape_url,
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
        profile: dto::ReqwestProfileDTO,
        skin: Option<&[u8]>,
        cape: Option<&[u8]>,
    ) -> Result<(), BackendError> {
        let mut update_user =
            users::Entity::update_many().filter(users::Column::Login.eq(user.login));

        if let Some(data) = skin {
            let hash =
                assets::service::upload_image(storage, assets::service::AssetType::Skin, data)
                    .await?;
            update_user = update_user.col_expr(users::Column::SkinHash, Expr::value(hash));
        }

        if let Some(data) = cape {
            let hash =
                assets::service::upload_image(storage, assets::service::AssetType::Cape, data)
                    .await?;
            update_user = update_user.col_expr(users::Column::CapeHash, Expr::value(hash));
        }

        update_user = update_user.col_expr(users::Column::IsAlex, Expr::value(profile.is_alex));

        update_user
            .exec(db)
            .await
            .map_err(|_| BackendError::InternalError)?;
        Ok(())
    }
}
