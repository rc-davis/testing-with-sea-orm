use bakery_backend::{Bakery, ContactDetails, Env};
use sea_orm::Database;
use std::error::Error;

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/bakeries_db";
// const DATABASE_URL: &str = "sqlite:./bakeries.db?mode=rwc"; // SQLite (in file)
// const DATABASE_URL: &str = "sqlite::memory:"; // SQLite (in memory)

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::connect(DATABASE_URL).await?;
    let env = Env::new(db);

    let insert_bakery = env.bakery_repository.insert("Happy Bakery", 0.0).await?;
    dbg!(&insert_bakery);

    let bakery_id = insert_bakery.last_insert_id;
    let bakery: Bakery = env
        .bakery_repository
        .update_name(bakery_id, "Happier Bakery")
        .await?;
    dbg!(&bakery);

    let insert_chef = env
        .chef_repository
        .insert(
            "John",
            &ContactDetails {
                email: "muffin.man@happier-bake.com".to_owned(),
                address: "123 Drury Lane".to_owned(),
            },
            bakery_id,
        )
        .await?;
    dbg!(&insert_chef);
    let chef_id = insert_chef.last_insert_id;

    let all_bakeries = env.bakery_repository.find_all().await?;
    dbg!(&all_bakeries);

    let happer_bakery_by_id = env.bakery_repository.find_by_id(bakery_id).await?;
    dbg!(&happer_bakery_by_id);

    let happer_bakery_by_name = env
        .bakery_repository
        .find_one_by_name("Happier Bakery")
        .await?;
    dbg!(&happer_bakery_by_name);

    let delete_chef = env.bakery_repository.delete_by_id(chef_id).await?;
    dbg!(&delete_chef);

    let delete_bakery = env.bakery_repository.delete_by_id(bakery_id).await?;
    dbg!(&delete_bakery);

    let no_bakeries = env.bakery_repository.find_all().await?;
    dbg!(&no_bakeries);

    Ok(())
}
