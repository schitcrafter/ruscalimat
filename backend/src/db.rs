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
    pub id: PrimaryKey,
    pub name: String,
    #[sqlx(rename = "type")]
    pub product_type: ProductType,
}

#[derive(SimpleObject)]
pub struct ProductWithFavorite {
    pub id: PrimaryKey,
    pub product: Product,
    pub is_favorite: bool,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProductType {
    HotDrink,
    ColdDrink,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "PurchaseInput")]
pub struct Purchase {
    pub id: PrimaryKey,
    pub account: Account,
    pub product: Product,
    pub quantity: u32,
}

#[derive(SimpleObject)]
pub struct PurchaseCount {
    pub id: PrimaryKey,
    pub product: Product,
    pub count: u64,
}
