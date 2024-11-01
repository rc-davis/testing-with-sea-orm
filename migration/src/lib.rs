// use clap::Parser;
// use dotenvy::dotenv;
use crate::sea_orm::DatabaseConnection;
pub use sea_orm_migration::{
    cli::{run_migrate, Cli},
    prelude::*,
};
use std::{error::Error, fmt::Display};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
// use sea_orm_cli::{run_migrate_generate, run_migrate_init, MigrateSubcommands};

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

pub async fn migrate(db: &DatabaseConnection, verbose: bool) {
    // dotenv().ok();
    // let cli = Cli::parse_from(vec!["prog", "migrate", "up"]);

    // let url = cli
    //     .database_url
    //     .expect("Environment variable 'DATABASE_URL' not set");
    // let schema = cli.database_schema.unwrap_or_else(|| );

    // let connect_options = ConnectOptions::new(TEST_DATABASE_URL)
    //     // .set_schema_search_path("public".to_owned())
    //     .to_owned();
    // let db = &Database::connect(connect_options)
    //     .await
    //     .expect("Failed to acquire database connection");

    let filter = match verbose {
        true => "debug",
        false => "sea_orm_migration=info",
    };

    let filter_layer = EnvFilter::try_new(filter).unwrap();

    if verbose {
        let fmt_layer = tracing_subscriber::fmt::layer();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init()
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_level(false)
            .without_time();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init()
    };

    Migrator::up(db, None).await.expect("Failed to migrate");
}
