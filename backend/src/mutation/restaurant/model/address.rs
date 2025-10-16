use async_graphql::InputObject;

#[derive(InputObject)]
pub struct AddressInput {
    pub address: String,
    pub complement: Option<String>,
    pub zip_code: String,
    pub city: String,
    pub country: String,
}
