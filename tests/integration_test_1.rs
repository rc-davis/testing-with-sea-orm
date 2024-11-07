use tracing::debug;

use testing_with_sea_orm::repository::*;

mod common;

#[tokio_shared_rt::test(shared)]
async fn can_insert_bakery() {
    let env = common::setup().await;
    bakery_repository::insert(&env.db, 0.0, "Happy Bakery")
        .await
        .expect("Failed to insert bakery");
}

#[tokio_shared_rt::test(shared)]
async fn can_retrieve_bakery() {
    let env = common::setup().await;
    let insert_bakery = bakery_repository::insert(&env.db, 0.0, "Happy Bakery")
        .await
        .expect("Failed to insert bakery");
    debug!("insert_result: {:?}", &insert_bakery);
    let bakery_id = insert_bakery.last_insert_id;

    let happer_bakery_by_id = bakery_repository::find_by_id(&env.db, bakery_id)
        .await
        .expect("Failed to find bakery by id");
    debug!("happer_bakery_by_id: {:?}", &happer_bakery_by_id);
}
