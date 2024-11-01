use sea_orm::*;

use super::entities::chef;
use crate::domain::contact_details::ContactDetails;

pub struct ChefRepository {
    db: DatabaseConnection,
}

impl ChefRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn insert(
        &self,
        name: &str,
        contact_details: &ContactDetails,
        bakery_id: i32,
    ) -> anyhow::Result<InsertResult<chef::ActiveModel>> {
        chef::Entity::insert(chef::ActiveModel {
            name: ActiveValue::Set(name.to_owned()),
            contact_details: ActiveValue::Set(Some(serde_json::to_value(contact_details)?)),
            bakery_id: ActiveValue::Set(bakery_id),
            ..Default::default()
        })
        .exec(&self.db)
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_by_id(&self, id: i32) -> anyhow::Result<DeleteResult> {
        chef::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        }
        .delete(&self.db)
        .await
        .map_err(|e| e.into())
    }
}
