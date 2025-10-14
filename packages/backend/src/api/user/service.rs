use crate::{
    BackendError,
    api::{
        assets::{self, service::AssetType},
        auth::dto::{LoginDTO, RegisterDTO},
        entities::users,
    },
};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
};
use serde::Serialize;

pub async fn find_user(
    db: &DatabaseConnection,
    data: &LoginDTO,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
        .filter(users::Column::Login.eq(&data.login))
        .one(db)
        .await
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

fn get_skin_data(user: users::Model) -> ProfileDTO {
    ProfileDTO {
        is_alex: user.is_alex,
        skin_url: assets::service::format_url(AssetType::Skin, &user.skin_hash.as_deref().unwrap()),
        cape_url: assets::service::format_url(AssetType::Cape, &user.cape_hash.as_deref().unwrap()),
    }
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
