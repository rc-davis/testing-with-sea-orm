use sea_orm::DatabaseConnection;

pub mod domain;
pub mod repository;

pub use domain::contact_details::*;
pub use repository::bakery_repository::*;
pub use repository::chef_repository::*;

pub struct Env {
    pub bakery_repository: BakeryRepository,
    pub chef_repository: ChefRepository,
}

impl Env {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            bakery_repository: BakeryRepository::new(&db),
            chef_repository: ChefRepository::new(&db),
        }
    }
}
