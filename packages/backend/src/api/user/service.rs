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
    uuid: String,
) -> Result<user::dto::ResponseProfileDTO, BackendError> {
    let user = find_user(db, users::Column::Uuid, &uuid)
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::BadRequest("User not found".into()))?;

    Ok(get_skin_data(user).await)
}

// TODO может тоже придётся переписать и заодно заменить AssetType на что то другое
async fn get_skin_data(user: users::Model) -> user::dto::ResponseProfileDTO {
    let skin_url = if let Some(hash) = user.skin_hash.as_deref() {
        assets::service::format_url(assets::service::AssetType::Skin, hash).await
    } else {
        None
    };

    let cape_url = if let Some(hash) = user.cape_hash.as_deref() {
        assets::service::format_url(assets::service::AssetType::Cape, hash).await
    } else {
        None
    };

    user::dto::ResponseProfileDTO {
        is_alex: user.is_alex,
        skin_url,
        cape_url,
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
