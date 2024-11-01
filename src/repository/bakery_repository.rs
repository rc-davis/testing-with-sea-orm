use sea_orm::*;

use super::entities::bakery;

pub use bakery::Model as Bakery;

pub struct BakeryRepository {
    db: DatabaseConnection,
}

impl BakeryRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn insert(
        &self,
        name: &str,
        profit_margin: f64,
    ) -> anyhow::Result<InsertResult<bakery::ActiveModel>> {
        bakery::Entity::insert(bakery::ActiveModel {
            name: ActiveValue::Set(name.to_owned()),
            profit_margin: ActiveValue::Set(profit_margin),
            ..Default::default()
        })
        .exec(&self.db)
        .await
        .map_err(|e| e.into())
    }

    pub async fn update_name(&self, id: i32, name: &str) -> anyhow::Result<Bakery> {
        bakery::ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(name.to_owned()),
            profit_margin: ActiveValue::NotSet,
        }
        .update(&self.db)
        .await
        .map_err(|e| e.into())
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<Bakery>> {
        bakery::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<Bakery>> {
        bakery::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn find_one_by_name(&self, name: &str) -> anyhow::Result<Option<Bakery>> {
        bakery::Entity::find()
            .filter(bakery::Column::Name.eq(name))
            .one(&self.db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_by_id(&self, id: i32) -> anyhow::Result<DeleteResult> {
        bakery::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        }
        .delete(&self.db)
        .await
        .map_err(|e| e.into())
    }
}
