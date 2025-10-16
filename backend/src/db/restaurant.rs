use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::db::Table;

#[derive(Serialize, Deserialize)]
pub struct Restaurant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub parent: Option<RecordId>,
    pub name: String,
    pub address: Address,
    pub contact: Contact,
}

impl Table for Restaurant {
    const TABLE_NAME: &str = "restaurant";
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub complement: Option<String>,
    pub zip_code: String,
    pub city: String,
    pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}
