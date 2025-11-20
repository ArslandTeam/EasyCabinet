use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(
                        ColumnDef::new(Users::Uuid)
                            .string_len(36)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Login)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string_len(60).not_null())
                    .col(ColumnDef::new(Users::AccessToken).string_len(255).null())
                    .col(ColumnDef::new(Users::ServerId).string_len(255).null())
                    .col(ColumnDef::new(Users::SkinHash).string_len(255).null())
                    .col(ColumnDef::new(Users::CapeHash).string_len(255).null())
                    .col(ColumnDef::new(Users::IsAlex).boolean().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "uuid")]
    Uuid,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "login")]
    Login,
    #[sea_orm(iden = "password")]
    Password,
    #[sea_orm(iden = "accessToken")]
    AccessToken,
    #[sea_orm(iden = "serverId")]
    ServerId,
    #[sea_orm(iden = "skinHash")]
    SkinHash,
    #[sea_orm(iden = "capeHash")]
    CapeHash,
    #[sea_orm(iden = "isAlex")]
    IsAlex,
}

// TODO add assert_eq!
