use crate::{
    db::{self, Table},
    jwt::{JWT_COOKIE_NAME, Jwt},
    mutation::MutationHandler,
};
use async_graphql::{InputObject, SimpleObject};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Duration;
use scrypt::{
    Scrypt,
    password_hash::{PasswordHash, PasswordVerifier},
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use time::OffsetDateTime;
use tokio::sync::mpsc::Sender;

#[derive(InputObject)]
pub struct LoginInput {
    #[graphql(validator(email))]
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginPayload {
    pub success: bool,
}

impl MutationHandler for LoginInput {
    type Response = LoginPayload;

    async fn handle<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> async_graphql::Result<Self::Response> {
        let db = ctx.data::<Surreal<Client>>()?;

        let user = db
            .query("SELECT * FROM type::table($table) WHERE email = $email")
            .bind(("table", db::user::User::TABLE_NAME))
            .bind(("email", self.email.to_owned()))
            .await?
            .take::<Option<db::user::User>>(0)?;
        if user.is_none() {
            return Err(async_graphql::Error::new("Email or password incorrect."));
        }
        let user = user.unwrap();

        let password_hash = PasswordHash::new(user.password.as_str())?;
        if Scrypt
            .verify_password(self.password.as_bytes(), &password_hash)
            .is_err()
        {
            return Err(async_graphql::Error::new("Email or password incorrect."));
        }

        let now = chrono::Utc::now();
        let exp = match now.checked_add_signed(Duration::days(1)) {
            Some(x) => x,
            None => {
                return Err(async_graphql::Error::new(
                    "Failed to generate JWT expiration time.",
                ));
            }
        };
        let jwt = Jwt {
            sub: user.id.unwrap().to_string(),
            iat: now,
            exp,
        };

        let cookie = Cookie::build((JWT_COOKIE_NAME, jwt.sign()?))
            .path("/")
            .http_only(true)
            .secure(true)
            .expires(OffsetDateTime::from_unix_timestamp(exp.timestamp())?)
            .build();
        if let Some(sender) = ctx.data_opt::<Sender<Cookie>>() {
            let _ = sender.send(cookie).await;
        }

        Ok(LoginPayload { success: true })
    }
}
