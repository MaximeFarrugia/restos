pub mod restaurant;
pub mod stock;
pub mod user;

pub trait Table {
    const TABLE_NAME: &str;
}
