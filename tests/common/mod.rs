use std::env;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use migration::test_migrate;
use sea_orm::{ConnectOptions, Database};
use testing_with_sea_orm::{setup_logging, Env};
use tracing::debug;

lazy_static! {
    static ref ENV: AsyncOnce<Env> = AsyncOnce::new(async {
        let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";
        let verbose =
            debug || env::var("VERBOSE").unwrap_or_else(|_| "false".to_string()) == "true";
        setup_logging(verbose);

        let url = if debug {
            let module_path = module_path!();
            let top_level_module = module_path.split("::").next().unwrap();
            format!("sqlite://{}.sqlite?mode=rwc", top_level_module)
        } else {
            "sqlite::memory:".to_owned()
        };
        debug!(?url);
        let opt = ConnectOptions::new(url)
            .set_schema_search_path("public")
            .to_owned();

        let db = Database::connect(opt)
            .await
            .expect("Failed to connect to database");
        test_migrate(&db).await;
        Env::new(db)
    });
}

pub async fn setup() -> &'static Env {
    ENV.get().await
}
