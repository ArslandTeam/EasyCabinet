// TODO переписать
use crate::{
    BackendError,
    api::{
        assets, auth,
        database::{
            entities::{sessions, users},
            service::DatabaseService,
        },
        storage::service::StorageService,
        user,
    },
    generate_config::CONFIG,
};
use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, InsertResult,
    QueryFilter,
};

#[derive(Default)]
pub struct UserService;

impl UserService {
    pub async fn get_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        uuid: String,
    ) -> Result<user::dto::ResponseProfileDTO, BackendError> {
        let user = DatabaseService::find_user(db, users::Column::Uuid, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        Ok(Self::get_textures_data(storage, &user).await)
    }

    pub async fn get_textures_data(
        storage: &StorageService,
        user: &users::Model,
    ) -> user::dto::ResponseProfileDTO {
        let skin = user
            .skin_hash
            .as_ref()
            .map(|hash| storage.format_url("skin", hash));
        let cape = user
            .cape_hash
            .as_ref()
            .map(|hash| storage.format_url("cape", hash));

        user::dto::ResponseProfileDTO {
            is_alex: user.is_alex,
            skin_url: skin,
            cape_url: cape,
        }
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        user: auth::jwt::JwtPayload,
        profile: user::dto::ReqwestProfileDTO,
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
