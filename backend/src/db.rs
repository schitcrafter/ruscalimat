use core::panic;

use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;

/// Currently a BIGINT
pub type PrimaryKey = i64;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "AccountInput")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    #[graphql(secret)]
    pub pin_hash: Option<String>,
    pub balance: i64,
}

#[derive(SimpleObject, InputObject, FromRow)]
#[graphql(input_name = "ProductInput")]
pub struct Product {
    #[graphql(default = -1)]
    pub id: PrimaryKey,
    pub name: String,
    pub product_type: ProductType,
    pub price: i64,
    pub picture: Option<String>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct ProductWithFavorite {
    #[graphql(default = -1)]
    pub id: PrimaryKey,
    pub name: String,
    pub product_type: ProductType,
    pub price: i64,
    pub picture: Option<String>,
    #[sqlx(default)]
    pub is_favorite: bool,
}

#[derive(async_graphql::Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
#[sqlx(type_name = "product_type", rename_all = "lowercase")]
pub enum ProductType {
    HotDrink,
    ColdDrink,
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
    pub account_id: String,
    pub product_id: PrimaryKey,
    pub paid_price: i64,
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
