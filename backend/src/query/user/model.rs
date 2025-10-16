use async_graphql::{ComplexObject, Context, Enum, SimpleObject, ID};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{db::{self, Table}, query::restaurant::model::Restaurant};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserRole {
    Owner,
}

impl From<db::user::UserRole> for UserRole {
    fn from(value: db::user::UserRole) -> Self {
        match value {
            db::user::UserRole::Owner => Self::Owner,
        }
    }
}

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub email: String,
    pub phone_number: Option<String>,
}

#[ComplexObject]
impl User {
    async fn restaurant<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Option<Restaurant>> {
        let db = ctx.data::<Surreal<Client>>()?;

        let res = db
            .query("SELECT parent FROM type::table($table) WHERE id = $id")
            .bind(("table", db::restaurant::Restaurant::TABLE_NAME))
            .bind(("id", self.id.to_string()))
            .await?
            .take::<Option<db::restaurant::Restaurant>>(0)?;

        Ok(res.and_then(|x| Some(x.into())))
    }
}

impl From<&db::user::User> for User {
    fn from(value: &db::user::User) -> Self {
        Self {
            id: value.id.clone().unwrap().into(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            role: value.role.clone().into(),
            email: value.email.clone(),
            phone_number: value.phone_number.clone(),
        }
    }
}

impl From<db::user::User> for User {
    fn from(value: db::user::User) -> Self {
        Self::from(&value)
    }
}
