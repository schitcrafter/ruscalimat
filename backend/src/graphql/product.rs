use async_graphql::Object;

use crate::db::{Product, ProductType, ProductWithFavorite};

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn products(&self) -> Vec<Product> {
        todo!()
    }

    async fn products_with_favorites(&self, _product_type: Option<ProductType>) -> Vec<Product> {
        todo!()
    }

    async fn product(&self, _id: u64) -> Product {
        todo!()
    }

    async fn product_with_favorite(&self, _id: u64) -> ProductWithFavorite {
        todo!()
    }
}

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    async fn create_product(&self, _product: Product) -> Product {
        todo!()
    }

    async fn update_product(&self, _product: Product) -> Product {
        todo!()
    }

    async fn delete_product(&self, _id: u64) -> bool {
        todo!()
    }

    async fn toggle_favorite(&self, _id: u64) -> bool {
        todo!()
    }
}
