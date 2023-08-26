use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::db::{PrimaryKey, Purchase};

use super::extract_user_claims;

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

    async fn my_purchases(&self, ctx: &Context<'_>) -> Result<Vec<Purchase>> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;

        sqlx::query_as!(
            Purchase,
            r#"SELECT * FROM purchases WHERE account_id = $1"#,
            user_claims.user_id
        )
        .fetch_all(db)
        .await
        .map_err(|e| e.extend_with(|_, e| e.set("code", 500)))
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
        sqlx::query_as!(
            Purchase,
            "SELECT * FROM purchases WHERE id = $1",
            product_id
        )
        .fetch_one(db)
        .await
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }
}

#[derive(Default)]
pub struct PurchaseMutation;

#[Object]
impl PurchaseMutation {
    async fn make_purchase(
        &self,
        ctx: &Context<'_>,
        product_id: PrimaryKey,
        quantity: i32,
    ) -> Result<Purchase> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;

        let product = sqlx::query!("SELECT price FROM products WHERE id = $1", product_id)
            .fetch_one(db)
            .await?;

        let paid_price: i64 = quantity as i64 * product.price;

        sqlx::query!(
            "UPDATE accounts SET balance = balance - $2 WHERE id = $1",
            user_claims.user_id,
            paid_price
        )
        .execute(db)
        .await?;

        let purchase = sqlx::query_as!(
            Purchase,
            r#"
            INSERT INTO purchases
            (account_id, product_id, quantity, paid_price)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
            user_claims.user_id,
            product_id,
            quantity,
            paid_price
        )
        .fetch_one(db)
        .await?;

        Ok(purchase)
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
