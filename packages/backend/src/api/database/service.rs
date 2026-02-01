use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, InsertResult,
    QueryFilter,
};

use crate::{
    api::database::entities::{sessions, users},
    generate_config::CONFIG,
};

pub struct DatabaseService;

impl DatabaseService {
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

    pub async fn create_session(
        db: &DatabaseConnection,
        uuid: String,
        user_agent: String,
    ) -> Result<InsertResult<sessions::ActiveModel>, DbErr> {
        let now = chrono::Utc::now();

        let session = sessions::ActiveModel {
            uuid: ActiveValue::Set(uuid),
            user_agent: ActiveValue::Set(user_agent),
            exp: ActiveValue::Set(
                now + chrono::Duration::seconds(CONFIG.cookie_expresion_in as i64),
            ),
            iat: ActiveValue::Set(now),
            ..Default::default()
        };

        sessions::Entity::insert(session).exec(db).await
    }

    pub async fn delete_session(db: &DatabaseConnection, session_id: i32) -> Result<(), DbErr> {
        sessions::Entity::delete_by_id(session_id).exec(db).await?;
        Ok(())
    }

    pub async fn delete_sessions(db: &DatabaseConnection, uuid: &str) -> Result<(), DbErr> {
        sessions::Entity::delete_many()
            .filter(sessions::Column::Uuid.eq(uuid))
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn get_sessions(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Vec<sessions::Model>, DbErr> {
        sessions::Entity::find()
            .filter(Expr::col(sessions::Column::Uuid).eq(uuid))
            .all(db)
            .await
    }
}
