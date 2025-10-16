use async_graphql::InputObject;

use crate::mutation::restaurant::model::AddressInput;
use crate::mutation::user::validator::PasswordValidator;

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
