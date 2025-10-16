use async_graphql::{Context, Object};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::{
    db::{self, Table},
    query::stock::model::Item,
};

#[derive(Default)]
pub struct StockQuery;

#[Object]
impl StockQuery {
    async fn items<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<Vec<Item>> {
        let db = ctx.data::<Surreal<Client>>()?;

        let res = db
            .select::<Vec<db::stock::Item>>(db::stock::Item::TABLE_NAME)
            .await?
            .iter()
            .map(Item::from)
            .collect::<Vec<_>>();

        Ok(res)
    }
}
