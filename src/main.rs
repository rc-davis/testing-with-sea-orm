use std::env;
use std::error::Error;

use sea_orm::{Database, TransactionTrait};
use tracing::info;

use testing_with_sea_orm::{repository::*, setup_logging, ContactDetails, Env};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/bakeries_db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let verbose = env::var("VERBOSE").unwrap_or_else(|_| "false".to_string()) == "true";

    setup_logging(verbose);

    let db = Database::connect(DATABASE_URL).await?;
    let env = Env::new(db);

    let txn = env.db.begin().await?;
    info!(?txn);

    let insert_bakery = bakery_repository::insert(&txn, 0.0, "Happy Bakery").await?;
    info!(?insert_bakery);
    txn.commit().await?;
    let bakery_id = insert_bakery.last_insert_id;

    let bakery: Bakery =
        bakery_repository::update_name(&env.db, "Happier Bakery", bakery_id).await?;
    info!(?bakery);

    let insert_chef = chef_repository::insert(
        &env.db,
        &ContactDetails {
            email: "muffin.man@happier-bake.com".to_owned(),
            address: "123 Drury Lane".to_owned(),
        },
        bakery_id,
        "John",
    )
    .await?;
    info!(?insert_chef);
    let chef_id = insert_chef.last_insert_id;

    let all_bakeries = bakery_repository::find_all(&env.db).await?;
    info!(?all_bakeries);

    let happer_bakery_by_id = bakery_repository::find_by_id(&env.db, bakery_id).await?;
    info!(?happer_bakery_by_id);

    let happer_bakery_by_name =
        bakery_repository::find_one_by_name(&env.db, "Happier Bakery").await?;
    info!(?happer_bakery_by_name);

    let delete_chef = chef_repository::delete_by_id(&env.db, chef_id).await?;
    info!(?delete_chef);

    let delete_bakery = bakery_repository::delete_by_id(&env.db, bakery_id).await?;
    info!(?delete_bakery);

    let no_bakeries = bakery_repository::find_all(&env.db).await?;
    info!(?no_bakeries);

    Ok(())
}
