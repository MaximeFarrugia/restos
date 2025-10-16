use async_graphql::{ComplexObject, Context, ID, SimpleObject};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::{
    db::{self, Table},
    query::restaurant::model::{Address, Contact},
};

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct Restaurant {
    pub id: ID,
    pub name: String,
    pub address: Address,
    pub contact: Contact,
}

#[ComplexObject]
impl Restaurant {
    async fn parent<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Option<Restaurant>> {
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

impl From<&db::restaurant::Restaurant> for Restaurant {
    fn from(value: &db::restaurant::Restaurant) -> Self {
        Self {
            id: value.id.clone().unwrap().into(),
            name: value.name.clone(),
            address: (&value.address).into(),
            contact: (&value.contact).into(),
        }
    }
}

impl From<db::restaurant::Restaurant> for Restaurant {
    fn from(value: db::restaurant::Restaurant) -> Self {
        Self::from(&value)
    }
}
