use crate::{
    auth::dto::{LoginDTO, RegisterDTO},
    entities::users,
};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
};

pub async fn find_user(
    db: &DatabaseConnection,
    data: &LoginDTO,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
        .filter(users::Column::Login.eq(&data.login))
        .filter(users::Column::Email.eq(&data.email))
        .one(db)
        .await
}

pub async fn create_user(
    db: &DatabaseConnection,
    data: RegisterDTO,
) -> Result<InsertResult<users::ActiveModel>, DbErr> {
    let user = users::ActiveModel {
        login: ActiveValue::Set(data.login),
        password: ActiveValue::Set(data.password),
        email: ActiveValue::Set(data.email),
        ..Default::default()
    };

    users::Entity::insert(user).exec(db).await
}
