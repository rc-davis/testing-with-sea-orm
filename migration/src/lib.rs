use crate::sea_orm::DatabaseConnection;
pub use sea_orm_migration::{
    cli::{run_migrate, Cli},
    prelude::*,
};

mod m20241031_000001_create_bakery_table;
mod m20241031_052346_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241031_000001_create_bakery_table::Migration),
            Box::new(m20241031_052346_create_chef_table::Migration),
        ]
    }
}

pub async fn test_migrate(db: &DatabaseConnection) {
    Migrator::fresh(db).await.expect("Failed to migrate");
}
