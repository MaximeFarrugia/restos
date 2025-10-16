use async_graphql::{ID, SimpleObject};

use crate::db;

#[derive(SimpleObject, Debug)]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub r#type: String,
    pub allergens: Vec<String>,
    pub quantity_unit: String,
}

impl From<&db::stock::Item> for Item {
    fn from(value: &db::stock::Item) -> Self {
        Self {
            id: value.id.clone().unwrap().into(),
            name: value.name.clone(),
            description: value.description.clone(),
            brand: value.brand.clone(),
            category: value.category.clone(),
            r#type: value.r#type.clone(),
            allergens: value.allergens.clone(),
            quantity_unit: value.quantity_unit.clone(),
        }
    }
}

impl From<db::stock::Item> for Item {
    fn from(value: db::stock::Item) -> Self {
        Self::from(&value)
    }
}
