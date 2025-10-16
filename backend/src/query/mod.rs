pub mod restaurant;
pub mod stock;
pub mod user;

use async_graphql::MergedObject;

use crate::query::stock::query::StockQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(StockQuery);
