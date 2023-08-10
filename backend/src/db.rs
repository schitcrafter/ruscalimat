use core::panic;

use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;

/// Currently a BIGINT
pub type PrimaryKey = i64;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "AccountInput")]
pub struct Account {
    pub id: PrimaryKey,
    pub name: String,
    pub email: String,
    pub deleted_at: Option<NaiveDateTime>,
    // #[graphql(secret)]
    pub pin_hash: String,
}

#[derive(SimpleObject, InputObject, FromRow)]
#[graphql(input_name = "ProductInput")]
pub struct Product {
    #[graphql(default = -1)]
    pub id: PrimaryKey,
    pub name: String,
    pub product_type: ProductType,
}

#[derive(SimpleObject)]
pub struct ProductWithFavorite {
    pub id: PrimaryKey,
    pub product: Product,
    pub is_favorite: bool,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
#[repr(i16)]
pub enum ProductType {
    HotDrink = 0,
    ColdDrink = 1,
}

impl From<i16> for ProductType {
    fn from(value: i16) -> ProductType {
        use ProductType::*;
        match value {
            0 => HotDrink,
            1 => ColdDrink,
            _ => panic!("Tried to convert {value} to ProductType, this shouldn't happen"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "PurchaseInput")]
pub struct Purchase {
    pub id: PrimaryKey,
    pub account_id: PrimaryKey,
    pub product_id: PrimaryKey,
    pub quantity: i32,
    pub refunded: bool,
}

#[derive(SimpleObject)]
pub struct ProductPurchaseCount {
    pub id: PrimaryKey,
    pub product: Product,
    pub count: i32,
}

#[derive(SimpleObject)]
pub struct AccountPurchaseCount {
    pub id: PrimaryKey,
    pub count: i32,
}
