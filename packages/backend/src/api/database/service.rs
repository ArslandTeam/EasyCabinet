use crate::{BackendError, api::database::entities::users};
use migration::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, InsertResult,
    QueryFilter, SqlErr,
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
    ) -> Result<InsertResult<users::ActiveModel>, BackendError> {
        let user = users::ActiveModel {
            uuid: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            login: ActiveValue::Set(login),
            password: ActiveValue::Set(password),
            email: ActiveValue::Set(email),
            ..Default::default()
        };

        users::Entity::insert(user).exec(db).await.map_err(|e| {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = e.sql_err() {
                return BackendError::BadRequest("User already exists".into());
            }
            BackendError::InternalError
        })
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
}
