use sea_orm::TransactionTrait;
use tracing::debug;

use testing_with_sea_orm::{repository::*, ContactDetails};

mod common;

#[tokio_shared_rt::test(shared)]
async fn can_update_bakery_name() {
    let env = common::setup().await;

    let insert_bakery = bakery_repository::insert(&env.db, 0.0, "Happy Bakery")
        .await
        .expect("Failed to insert bakery");
    debug!("insert_bakery: {:?}", &insert_bakery);

    let bakery_id = insert_bakery.last_insert_id;
    let bakery: Bakery = bakery_repository::update_name(&env.db, "Happier Bakery", bakery_id)
        .await
        .expect("Failed to update bakery name");
    debug!("bakery: {:?}", &bakery);
    assert_eq!(bakery.name, "Happier Bakery");
}

#[tokio_shared_rt::test(shared)]
async fn can_switch_bakeries() {
    let env = common::setup().await;

    let insert_bakery1 = bakery_repository::insert(&env.db, 0.0, "Happy Bakery")
        .await
        .expect("Failed to insert bakery1");
    debug!("insert_bakery1: {:?}", &insert_bakery1);
    let bakery1_id = insert_bakery1.last_insert_id;

    let insert_chef = chef_repository::insert(
        &env.db,
        &ContactDetails {
            email: "muffin.man@happier-bake.com".to_owned(),
            address: "123 Drury Lane".to_owned(),
        },
        bakery1_id,
        "John",
    )
    .await
    .expect("Failed to insert chef");
    debug!("insert_chef: {:?}", &insert_chef);
    let chef_id = insert_chef.last_insert_id;

    let insert_bakery2 = bakery_repository::insert(&env.db, 0.1, "Happy Bakery")
        .await
        .expect("Failed to insert bakery2");
    debug!("insert_bakery2: {:?}", &insert_bakery2);
    let bakery2_id = insert_bakery2.last_insert_id;

    let chef: Chef = chef_repository::update_bakery(&env.db, bakery2_id, chef_id)
        .await
        .expect("Failed to update bakery for chef");
    debug!("chef: {:?}", &chef);
    assert_eq!(chef.bakery_id, bakery2_id);
}

#[tokio_shared_rt::test(shared)]
async fn can_roll_back_transaction() {
    let env = common::setup().await;

    let insert_bakery1 = bakery_repository::insert(&env.db, 0.0, "Happy Bakery")
        .await
        .expect("Failed to insert bakery1");
    debug!("insert_bakery1: {:?}", &insert_bakery1);
    let bakery1_id = insert_bakery1.last_insert_id;

    let insert_chef = chef_repository::insert(
        &env.db,
        &ContactDetails {
            email: "muffin.man@happier-bake.com".to_owned(),
            address: "123 Drury Lane".to_owned(),
        },
        bakery1_id,
        "John",
    )
    .await
    .expect("Failed to insert chef");
    debug!("insert_chef: {:?}", &insert_chef);
    let chef_id = insert_chef.last_insert_id;

    let txn = env.db.begin().await.expect("Failed to begin transaction");
    debug!("txn: {:?}", &txn);

    let insert_bakery2 = bakery_repository::insert(&txn, 0.1, "Happy Bakery")
        .await
        .expect("Failed to insert bakery2");
    debug!("insert_bakery2: {:?}", &insert_bakery2);
    let bakery2_id = insert_bakery2.last_insert_id;

    let chef: Chef = chef_repository::update_bakery(&txn, bakery2_id, chef_id)
        .await
        .expect("Failed to update bakery for chef");
    debug!("chef: {:?}", &chef);

    txn.rollback()
        .await
        .expect("Failed to rollback transaction");

    let chef_after_rollback: Chef = chef_repository::find_by_id(&env.db, chef_id)
        .await
        .expect("Failed to find chef by id")
        .expect("Chef not found");
    debug!("chef_after_rollback: {:?}", &chef_after_rollback);
    assert_eq!(chef_after_rollback.bakery_id, bakery1_id);
}
