use async_graphql::SimpleObject;

use crate::db::{self};

#[derive(SimpleObject, Debug)]
pub struct Address {
    pub address: String,
    pub complement: Option<String>,
    pub zip_code: String,
    pub city: String,
    pub country: String,
}

impl From<&db::restaurant::Address> for Address {
    fn from(value: &db::restaurant::Address) -> Self {
        Self {
            address: value.address.clone(),
            complement: value.complement.clone(),
            zip_code: value.zip_code.clone(),
            city: value.city.clone(),
            country: value.country.clone(),
        }
    }
}

impl From<db::restaurant::Address> for Address {
    fn from(value: db::restaurant::Address) -> Self {
        Self::from(&value)
    }
}
