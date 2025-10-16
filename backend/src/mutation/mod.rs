use async_graphql::{Context, MergedObject};

use crate::mutation::user::UserMutation;

pub mod restaurant;
pub mod user;

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation);

pub trait MutationHandler {
    type Response;

    async fn handle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<Self::Response>;
}
