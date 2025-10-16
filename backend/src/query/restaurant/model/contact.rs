use async_graphql::SimpleObject;

use crate::db::{self};

#[derive(SimpleObject, Debug)]
pub struct Contact {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

impl From<&db::restaurant::Contact> for Contact {
    fn from(value: &db::restaurant::Contact) -> Self {
        Self {
            email: value.email.clone(),
            phone_number: value.phone_number.clone(),
        }
    }
}

impl From<db::restaurant::Contact> for Contact {
    fn from(value: db::restaurant::Contact) -> Self {
        Self::from(&value)
    }
}
