pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250914_142509_users::Migration),
            Box::new(m20260119_165456_sessions::Migration),
        ]
    }
}
mod m20250914_142509_users;
mod m20260119_165456_sessions;
