mod login;
pub mod model;
mod register;
mod validator;

use async_graphql::{Context, Object};

use crate::{
    mutation::{user::{login::{LoginInput, LoginPayload}, register::RegisterInput}, MutationHandler},
    query::user::model::User,
};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: RegisterInput,
    ) -> async_graphql::Result<User> {
        input.handle(ctx).await
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: LoginInput,
    ) -> async_graphql::Result<LoginPayload> {
        input.handle(ctx).await
    }
}
