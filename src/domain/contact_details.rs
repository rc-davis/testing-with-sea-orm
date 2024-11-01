use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContactDetails {
    pub email: String,
    pub address: String,
}
