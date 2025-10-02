use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::users;

pub async fn find_user(
    db: &DatabaseConnection,
    login: String,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
        .filter(users::Column::Login.eq(login))
        .one(db)
        .await
}
