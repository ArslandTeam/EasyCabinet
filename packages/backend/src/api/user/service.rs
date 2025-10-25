// TODO переписать
use crate::{
    BackendError,
    api::{assets, auth, entities::users, user},
};
use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
};

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
    login: String,
    password: String,
    email: String,
) -> Result<InsertResult<users::ActiveModel>, DbErr> {
    let user = users::ActiveModel {
        uuid: ActiveValue::Set(Some(uuid::Uuid::new_v4().to_string())),
        login: ActiveValue::Set(login),
        password: ActiveValue::Set(password),
        email: ActiveValue::Set(email),
        ..Default::default()
    };

    users::Entity::insert(user).exec(db).await
}

pub async fn get_profile(
    db: &DatabaseConnection,
    uuid: String,
) -> Result<user::dto::ResponseProfileDTO, BackendError> {
    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(uuid))
        .one(db)
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::BadRequest("User not found".to_string()))?;

    Ok(get_skin_data(user))
}

// TODO может тоже придётся переписать и заодно заменить AssetType на что то другое
fn get_skin_data(user: users::Model) -> user::dto::ResponseProfileDTO {
    user::dto::ResponseProfileDTO {
        is_alex: user.is_alex,
        skin_url: user
            .skin_hash
            .as_deref()
            .and_then(|hash| assets::service::format_url(assets::service::AssetType::Skin, hash)),
        cape_url: user
            .cape_hash
            .as_deref()
            .and_then(|hash| assets::service::format_url(assets::service::AssetType::Cape, hash)),
    }
}

// FIX это пиздец я потом исправлю
pub async fn update_profile(
    db: &DatabaseConnection,
    user: auth::jwt::JwtPayload,
    profile: user::dto::ReqwestProfileDTO,
    skin: Option<&[u8]>,
    cape: Option<&[u8]>,
) -> Result<(), BackendError> {
    let mut update_user = users::Entity::update_many().filter(users::Column::Login.eq(user.login));

    if let Some(data) = skin {
        let hash = assets::service::upload_image(assets::service::AssetType::Skin, data).await?;
        update_user = update_user.col_expr(users::Column::SkinHash, Expr::value(hash));
    }

    if let Some(data) = cape {
        let hash = assets::service::upload_image(assets::service::AssetType::Cape, data).await?;
        update_user = update_user.col_expr(users::Column::CapeHash, Expr::value(hash));
    }

    update_user = update_user.col_expr(users::Column::IsAlex, Expr::value(profile.is_alex));

    update_user
        .exec(db)
        .await
        .map_err(|_| BackendError::InternalError)?;
    Ok(())
}
