use bakery_backend::Env;
use migration::migrate;
use sea_orm::Database;

const DATABASE_URL: &str = "sqlite::memory:";
const VERBOSE: bool = false;

#[tokio::test]
async fn can_insert_bakery() {
    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Failed to connect to database");
    migrate(&db, VERBOSE).await;
    let env = Env::new(db);

    env.bakery_repository
        .insert("Happy Bakery", 0.0)
        .await
        .expect("Failed to insert bakery");
}
