use async_graphql::Object;

use crate::db::Purchase;

#[derive(Default)]
pub struct PurchaseQuery;

#[Object]
impl PurchaseQuery {
    async fn purchases(&self) -> Vec<Purchase> {
        todo!()
    }

    async fn my_purchases(&self) -> Vec<Purchase> {
        todo!()
    }

    async fn purchase(&self, _id: u64) -> Purchase {
        todo!()
    }
}

#[derive(Default)]
pub struct PurchaseMutation;

#[Object]
impl PurchaseMutation {
    async fn make_purchase(&self, _product_id: u64, amount: u32) -> Purchase {
        todo!()
    }

    async fn refund_purchase(&self, _id: u64) -> bool {
        todo!()
    }
}
