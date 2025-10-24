// TODO переписать
use crate::{
    BackendError,
    api::{
        assets::{self, service::AssetType},
        auth::{dto::RegisterDTO, jwt::JwtPayload},
        entities::users,
    },
};
use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
};
use serde::Serialize;

pub async fn find_user(
    db: &DatabaseConnection,
    column: users::Column,
    value: String,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
        .filter(Expr::col(column).eq(value))
        .one(db)
        .await
}

pub async fn update_user_reset_token(
    db: &DatabaseConnection,
    email: String,
    reset_token: String,
) -> Result<(), DbErr> {
    users::Entity::update_many()
        .col_expr(users::Column::ResetToken, Expr::value(reset_token))
        .filter(users::Column::Email.eq(email))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn change_user_password(
    db: &DatabaseConnection,
    reset_token: String,
    password: String,
) -> Result<(), DbErr> {
    users::Entity::update_many()
        .col_expr(users::Column::Password, Expr::value(password))
        .filter(users::Column::ResetToken.eq(reset_token))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn create_user(
    db: &DatabaseConnection,
    data: RegisterDTO,
) -> Result<InsertResult<users::ActiveModel>, DbErr> {
    let user = users::ActiveModel {
        uuid: ActiveValue::Set(Some(uuid::Uuid::new_v4().to_string())),
        login: ActiveValue::Set(data.login),
        password: ActiveValue::Set(data.password),
        email: ActiveValue::Set(data.email),
        ..Default::default()
    };

    users::Entity::insert(user).exec(db).await
}

pub async fn get_profile(
    db: &DatabaseConnection,
    uuid: String,
) -> Result<ProfileDTO, BackendError> {
    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(uuid))
        .one(db)
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::BadRequest("User not found".to_string()))?;

    Ok(get_skin_data(user))
}

// TODO может тоже придётся переписать и заодно заменить AssetType на что то другое
fn get_skin_data(user: users::Model) -> ProfileDTO {
    ProfileDTO {
        is_alex: user.is_alex,
        skin_url: user
            .skin_hash
            .as_deref()
            .and_then(|hash| assets::service::format_url(AssetType::Skin, hash)),
        cape_url: user
            .cape_hash
            .as_deref()
            .and_then(|hash| assets::service::format_url(AssetType::Cape, hash)),
    }
}

// FIX это пиздец я потом исправлю
pub async fn update_profile(
    user: JwtPayload,
    db: &DatabaseConnection,
    skin: Option<&[u8]>,
    cape: Option<&[u8]>,
) -> Result<(), BackendError> {
    let mut update_user = users::Entity::update_many().filter(users::Column::Login.eq(user.login));

    if let Some(data) = skin {
        let hash = assets::service::upload_image(AssetType::Skin, data).await?;
        update_user = update_user.col_expr(users::Column::SkinHash, Expr::value(hash));
    }

    if let Some(data) = cape {
        let hash = assets::service::upload_image(AssetType::Cape, data).await?;
        update_user = update_user.col_expr(users::Column::CapeHash, Expr::value(hash));
    }

    update_user
        .exec(db)
        .await
        .map_err(|_| BackendError::InternalError)?;
    Ok(())
}

#[derive(Serialize)]
pub struct ProfileDTO {
    #[serde(rename = "isAlex")]
    pub is_alex: Option<bool>,
    #[serde(rename = "skinUrl")]
    pub skin_url: Option<String>,
    #[serde(rename = "capeUrl")]
    pub cape_url: Option<String>,
}
