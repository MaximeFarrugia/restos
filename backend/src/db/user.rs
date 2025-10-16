use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::db::Table;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Owner,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub first_name: String,
    pub last_name: String,
    pub restaurant: RecordId,
    pub role: UserRole,
    pub email: String,
    pub password: String,
    pub last_password_update: surrealdb::Datetime,
    pub phone_number: Option<String>,
}

impl Table for User {
    const TABLE_NAME: &str = "user";
}
