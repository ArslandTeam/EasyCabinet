use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: String,
    pub refresh_token: String,
    pub user_agent: String,
    pub exp: DateTimeUtc,
    pub iat: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
