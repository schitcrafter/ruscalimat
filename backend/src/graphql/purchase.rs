use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::db::{PrimaryKey, Purchase};

#[derive(Default)]
pub struct PurchaseQuery;

#[Object]
impl PurchaseQuery {
    async fn purchases(&self, ctx: &Context<'_>) -> Result<Vec<Purchase>> {
        let db = ctx.data()?;
        sqlx::query_as!(Purchase, "SELECT * FROM purchases")
            .fetch_all(db)
            .await
            .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }

    async fn my_purchases(&self) -> Vec<Purchase> {
        todo!()
    }

    async fn purchase(
        &self,
        ctx: &Context<'_>,
        product_id: PrimaryKey,
        amount: u32,
    ) -> Result<Purchase> {
        if amount == 0 {
            return Err(async_graphql::Error::new("amount cannot be 0"));
        }
        let db = ctx.data()?;
        sqlx::query_as!(Purchase, "SELECT * FROM purchases WHERE id = $1", id)
            .fetch_one(db)
            .await
            .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }
}

#[derive(Default)]
pub struct PurchaseMutation;

#[Object]
impl PurchaseMutation {
    async fn make_purchase(&self, _product_id: PrimaryKey, _amount: i32) -> Result<Purchase> {
        todo!()
    }

    async fn refund_purchase(&self, ctx: &Context<'_>, id: PrimaryKey) -> Result<bool> {
        let db = ctx.data()?;
        sqlx::query!("UPDATE purchases SET refunded = true WHERE id = $1", id)
            .execute(db)
            .await
            .map(|result| result.rows_affected() == 1)
            .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }
}
