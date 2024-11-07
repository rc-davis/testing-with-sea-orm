use sea_orm::DatabaseConnection;

pub mod domain;
mod log;
pub mod repository;

pub use domain::contact_details::*;
pub use log::*;

pub struct Env {
    pub db: DatabaseConnection,
}

impl Env {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
