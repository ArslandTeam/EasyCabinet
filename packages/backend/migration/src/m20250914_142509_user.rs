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
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(
                        ColumnDef::new(User::Uuid)
                            .string_len(36)
                            .unique_key()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Login)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Password).string_len(60).not_null())
                    .col(
                        ColumnDef::new(User::ResetToken)
                            .string_len(255)
                            .unique_key()
                            .null(),
                    )
                    .col(ColumnDef::new(User::AccessToken).string_len(255).null())
                    .col(ColumnDef::new(User::ServerId).string_len(255).null())
                    .col(ColumnDef::new(User::SkinHash).string_len(255).null())
                    .col(ColumnDef::new(User::CapeHash).string_len(255).null())
                    .col(ColumnDef::new(User::IsAlex).boolean().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
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
    #[sea_orm(iden = "resetToken")]
    ResetToken,
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
