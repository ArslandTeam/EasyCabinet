use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub login: String,
    pub password: String,
    #[sea_orm(column_name = "accessToken")]
    pub access_token: Option<String>,
    #[sea_orm(column_name = "serverId")]
    pub server_id: Option<String>,
    #[sea_orm(column_name = "skinHash")]
    pub skin_hash: Option<String>,
    #[sea_orm(column_name = "capeHash")]
    pub cape_hash: Option<String>,
    #[sea_orm(column_name = "isAlex")]
    pub is_alex: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::users::Entity")]
    // Users,
}

// impl Related<super::users::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Users.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
