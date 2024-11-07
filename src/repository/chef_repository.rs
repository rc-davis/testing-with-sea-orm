use super::entities::chef;
use crate::domain::contact_details::ContactDetails;
use sea_orm::*;

pub use chef::Model as Chef;

pub async fn insert<C: ConnectionTrait>(
    conn: &C,
    contact_details: &ContactDetails,
    bakery_id: i32,
    name: &str,
) -> anyhow::Result<InsertResult<chef::ActiveModel>> {
    chef::Entity::insert(chef::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        contact_details: ActiveValue::Set(Some(serde_json::to_value(contact_details)?)),
        bakery_id: ActiveValue::Set(bakery_id),
        ..Default::default()
    })
    .exec(conn)
    .await
    .map_err(|e| e.into())
}

pub async fn update_bakery<C: ConnectionTrait>(
    conn: &C,
    bakery_id: i32,
    id: i32,
) -> anyhow::Result<Chef> {
    chef::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::NotSet,
        contact_details: ActiveValue::NotSet,
        bakery_id: ActiveValue::Set(bakery_id),
    }
    .update(conn)
    .await
    .map_err(|e| e.into())
}

pub async fn find_by_id<C: ConnectionTrait>(conn: &C, id: i32) -> anyhow::Result<Option<Chef>> {
    chef::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(|e| e.into())
}
pub async fn delete_by_id<C: ConnectionTrait>(conn: &C, id: i32) -> anyhow::Result<DeleteResult> {
    chef::ActiveModel {
        id: ActiveValue::Set(id),
        ..Default::default()
    }
    .delete(conn)
    .await
    .map_err(|e| e.into())
}
