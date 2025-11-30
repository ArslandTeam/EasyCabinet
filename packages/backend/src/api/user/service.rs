// TODO переписать
use crate::{
    BackendError,
    api::{assets, auth, entities::users, storage::service::StorageService, user},
};
use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
};

#[derive(Default)]
pub struct UserService;

impl UserService {
    pub async fn find_user(
        db: &DatabaseConnection,
        column: users::Column,
        value: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(Expr::col(column).eq(value))
            .one(db)
            .await
    }

    pub async fn find_users(
        db: &DatabaseConnection,
        column: users::Column,
        value: Vec<String>,
    ) -> Result<Vec<users::Model>, DbErr> {
        users::Entity::find()
            .filter(Expr::col(column).is_in(value))
            .all(db)
            .await
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        column: users::Column,
        value: &str,
        column_filter: users::Column,
        filter: &str,
    ) -> Result<(), DbErr> {
        users::Entity::update_many()
            .col_expr(column, Expr::value(value))
            .filter(column_filter.eq(filter))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        login: String,
        password: String,
        email: String,
    ) -> Result<InsertResult<users::ActiveModel>, DbErr> {
        let user = users::ActiveModel {
            uuid: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            login: ActiveValue::Set(login),
            password: ActiveValue::Set(password),
            email: ActiveValue::Set(email),
            ..Default::default()
        };

        users::Entity::insert(user).exec(db).await
    }

    pub async fn get_profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        uuid: String,
    ) -> Result<user::dto::ResponseProfileDTO, BackendError> {
        let user = Self::find_user(db, users::Column::Uuid, &uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequest("User not found".into()))?;

        Ok(Self::get_skin_data(storage, user).await)
    }

    async fn get_skin_data(
        storage: &StorageService,
        user: users::Model,
    ) -> user::dto::ResponseProfileDTO {
        use base64::Engine;

        // TODO Думаю луше объеденить два if в один передавя лишь нужный тип текстуры
        let skin = if let Some(hash) = user.skin_hash {
            storage.get_file_bit("skin", &hash).await.ok().map(|bytes| {
                let textures = base64::engine::general_purpose::STANDARD.encode(&bytes);
                format!("data:image/png;base64,{textures}")
            })
        } else {
            None
        };

        let cape = if let Some(hash) = user.cape_hash {
            storage.get_file_bit("cape", &hash).await.ok().map(|bytes| {
                let textures = base64::engine::general_purpose::STANDARD.encode(&bytes);
                format!("data:image/png;base64,{textures}")
            })
        } else {
            None
        };

        user::dto::ResponseProfileDTO {
            is_alex: user.is_alex,
            skin_url: skin,
            cape_url: cape,
        }
    }

    // FIX это пиздец я потом исправлю
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
