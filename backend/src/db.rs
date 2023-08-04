use async_graphql::{InputObject, SimpleObject, Enum};


#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "AccountInput")]
pub struct Account {
    name: String,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "ProductInput")]
pub struct Product {
    name: String,
}

#[derive(SimpleObject)]
pub struct ProductWithFavorite {
    product: Product,
    is_favorite: bool,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProductType {
    HotDrink,
    ColdDrink,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "PurchaseInput")]
pub struct Purchase {
    account: Account,
    product: Product,
}

#[derive(SimpleObject)]
pub struct PurchaseCount {
    product: Product,
    count: u64,
}
