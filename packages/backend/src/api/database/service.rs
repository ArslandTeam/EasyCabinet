use crate::{BackendError, api::database::entities::users};
use migration::{Expr, IntoCondition};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SqlErr};

pub struct DatabaseService;

impl DatabaseService {
    pub async fn find<T>(
        db: &DatabaseConnection,
        column: T::Column,
        value: &str,
    ) -> Result<Option<T::Model>, BackendError>
    where
        T: EntityTrait,
        T::Column: ColumnTrait,
    {
        T::find()
            .filter(column.eq(value))
            .one(db)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))
    }

    pub async fn find_all<T, C>(
        db: &DatabaseConnection,
        filters: C,
    ) -> Result<Vec<T::Model>, BackendError>
    where
        T: EntityTrait,
        C: IntoCondition,
    {
        T::find()
            .filter(filters)
            .all(db)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        login: &str,
        password: Option<&str>,
        email: &str,
        discord_id: Option<&str>,
    ) -> Result<users::Model, BackendError> {
        let user = users::ActiveModel {
            uuid: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            discord_id: ActiveValue::Set(discord_id.map(|s| s.to_string())),
            login: ActiveValue::Set(login.to_string()),
            password: ActiveValue::Set(password.map(|s| s.to_string())),
            email: ActiveValue::Set(email.to_string()),
            ..Default::default()
        };

        users::Entity::insert(user)
            .exec_with_returning(db)
            .await
            .map_err(|e| {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = e.sql_err() {
                    return BackendError::BadRequest("User already exists".into());
                }
                BackendError::InternalError(e.into())
            })
    }

    /// Be careful with `Option<T>`. Mandatory parameters are processed in advance.
    pub async fn update_user(
        db: &DatabaseConnection,
        column: users::Column,
        value: Option<&str>,
        column_filter: users::Column,
        filter: &str,
    ) -> Result<(), BackendError> {
        users::Entity::update_many()
            .col_expr(column, Expr::value(value))
            .filter(column_filter.eq(filter))
            .exec(db)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))?;
        Ok(())
    }
}
