use sea_orm::*;

use super::entities::bakery;

pub use bakery::Model as Bakery;

pub async fn insert<C: ConnectionTrait>(
    conn: &C,
    profit_margin: f64,
    name: &str,
) -> anyhow::Result<InsertResult<bakery::ActiveModel>> {
    bakery::Entity::insert(bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(profit_margin),
        ..Default::default()
    })
    .exec(conn)
    .await
    .map_err(|e| e.into())
}

pub async fn update_name<C: ConnectionTrait>(
    conn: &C,
    name: &str,
    id: i32,
) -> anyhow::Result<Bakery> {
    bakery::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::NotSet,
    }
    .update(conn)
    .await
    .map_err(|e| e.into())
}

pub async fn find_all<C: ConnectionTrait>(conn: &C) -> anyhow::Result<Vec<Bakery>> {
    bakery::Entity::find().all(conn).await.map_err(|e| e.into())
}

pub async fn find_by_id<C: ConnectionTrait>(conn: &C, id: i32) -> anyhow::Result<Option<Bakery>> {
    bakery::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn find_one_by_name<C: ConnectionTrait>(
    conn: &C,
    name: &str,
) -> anyhow::Result<Option<Bakery>> {
    bakery::Entity::find()
        .filter(bakery::Column::Name.eq(name))
        .one(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn delete_by_id<C: ConnectionTrait>(conn: &C, id: i32) -> anyhow::Result<DeleteResult> {
    bakery::ActiveModel {
        id: ActiveValue::Set(id),
        ..Default::default()
    }
    .delete(conn)
    .await
    .map_err(|e| e.into())
}
