use crate::{
    db::{self, Table},
    mutation::{
        MutationHandler, restaurant::model::AddressInput, user::validator::PasswordValidator,
    },
    query::user::model::User,
};
use async_graphql::InputObject;
use scrypt::{
    Scrypt,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(InputObject)]
pub struct RegisterInput {
    pub first_name: String,
    pub last_name: String,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(custom = "PasswordValidator"))]
    pub password: String,
    pub phone_number: Option<String>,
    pub restaurant_name: String,
    pub restaurant_address: AddressInput,
    pub restaurant_contact_email: Option<String>,
    pub restaurant_contact_phone_number: Option<String>,
}

impl MutationHandler for RegisterInput {
    type Response = User;

    async fn handle<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> async_graphql::Result<Self::Response> {
        let db = ctx.data::<Surreal<Client>>()?;

        let restaurant = db
            .create::<Option<db::restaurant::Restaurant>>(db::restaurant::Restaurant::TABLE_NAME)
            .content(db::restaurant::Restaurant {
                id: None,
                parent: None,
                name: self.restaurant_name.to_owned(),
                address: db::restaurant::Address {
                    address: self.restaurant_address.address.to_owned(),
                    complement: self.restaurant_address.complement.to_owned(),
                    zip_code: self.restaurant_address.zip_code.to_owned(),
                    city: self.restaurant_address.city.to_owned(),
                    country: self.restaurant_address.country.to_owned(),
                },
                contact: db::restaurant::Contact {
                    email: self.restaurant_contact_email.clone(),
                    phone_number: self.restaurant_contact_phone_number.clone(),
                },
            })
            .await?;
        let restaurant = match restaurant {
            Some(restaurant) => restaurant,
            None => {
                return Err(async_graphql::Error::new("Failed to create restaurant."));
            }
        };

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(self.password.as_bytes(), &salt)?
            .to_string();

        let user = db
            .create::<Option<db::user::User>>(db::user::User::TABLE_NAME)
            .content(db::user::User {
                id: None,
                first_name: self.first_name.to_owned(),
                last_name: self.last_name.to_owned(),
                restaurant: restaurant.id.clone().unwrap(),
                role: db::user::UserRole::Owner,
                email: self.email.to_owned(),
                password: password_hash,
                last_password_update: chrono::Utc::now().into(),
                phone_number: self.phone_number.to_owned(),
            })
            .await;
        let user = match user {
            Ok(Some(user)) => user,
            Ok(None) => {
                let _ = db
                    .delete::<Option<db::restaurant::Restaurant>>(restaurant.id.unwrap())
                    .await;
                return Err(async_graphql::Error::new("Failed to create user."));
            }
            Err(e) => {
                let _ = db
                    .delete::<Option<db::restaurant::Restaurant>>(restaurant.id.unwrap())
                    .await;
                return Err(e.into());
            }
        };

        Ok(user.into())
    }
}
