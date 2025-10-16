use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::db::Table;

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub restaurant: RecordId,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub r#type: String,
    pub allergens: Vec<String>,
    pub quantity_unit: String,
}

impl Table for Item {
    const TABLE_NAME: &str = "item";
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub restaurant: RecordId,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Vec<String>,
    pub servings: f64,
}

impl Table for Recipe {
    const TABLE_NAME: &str = "recipe";
}

#[derive(Serialize, Deserialize)]
pub struct Stock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub restaurant: RecordId,
    pub item: RecordId,
    pub quantity: f64,
    pub low_quantity_threshold: Option<f64>,
    pub production_date: Option<surrealdb::Datetime>,
    pub use_by_date: Option<surrealdb::Datetime>,
    pub status: String,
    pub storage: String,
    pub supplier: Option<String>,
    pub cost: f64,
}

impl Table for Stock {
    const TABLE_NAME: &str = "stock";
}
