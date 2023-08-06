use async_graphql::{
    ContainerType, Enum, InputObject, InputObjectType, InputType, ObjectType, OutputType,
    SimpleObject,
};
use chrono::NaiveDateTime;

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

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "ProductInput")]
pub struct Product {
    pub id: PrimaryKey,
    pub name: String,
}

#[derive(SimpleObject)]
pub struct ProductWithFavorite {
    pub id: PrimaryKey,
    pub product: Product,
    pub is_favorite: bool,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
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
}

#[derive(SimpleObject)]
pub struct PurchaseCount {
    pub id: PrimaryKey,
    pub product: Product,
    pub count: u64,
}
