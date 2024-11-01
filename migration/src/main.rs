use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    cli::run_cli(migration::Migrator).await;
}
