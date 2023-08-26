use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::db::{PrimaryKey, Product, ProductType, ProductWithFavorite};

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data()?;
        sqlx::query_as!(
            Product,
            r#"
        SELECT id, name, price, product_type as "product_type: ProductType"
        FROM products
            "#
        )
        .fetch_all(db)
        .await
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }

    async fn products_with_favorites(&self, _product_type: Option<ProductType>) -> Vec<Product> {
        todo!()
    }

    async fn product(&self, ctx: &Context<'_>, id: PrimaryKey) -> Result<Product> {
        let db = ctx.data()?;
        sqlx::query_as!(
            Product,
            r#"
        SELECT id, name, price, product_type as "product_type: ProductType"
        FROM products
        WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }

    async fn product_with_favorite(&self, _id: u64) -> ProductWithFavorite {
        todo!()
    }
}

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    /// the field id on the input object here is ignored and optional
    async fn create_product(&self, ctx: &Context<'_>, product: Product) -> Result<Product> {
        let db = ctx.data()?;
        sqlx::query_as!(
            Product,
            r#"
        INSERT INTO products ( name, product_type, price )
        VALUES ( $1, $2, $3 )
        RETURNING id, name, price, product_type AS "product_type!: ProductType"
            "#,
            product.name,
            product.product_type as ProductType,
            product.price
        )
        .fetch_one(db)
        .await
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }

    async fn update_product(&self, ctx: &Context<'_>, product: Product) -> Result<Product> {
        let db = ctx.data()?;
        sqlx::query_as!(
            Product,
            r#"
            UPDATE products
            SET name = $2, product_type = $3, price = $4
            WHERE id = $1
            RETURNING id, name, price, product_type AS "product_type!: ProductType"
            "#,
            product.id,
            product.name,
            product.product_type as ProductType,
            product.price
        )
        .fetch_one(db)
        .await
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
    }

    async fn delete_product(&self, ctx: &Context<'_>, id: PrimaryKey) -> Result<bool> {
        let db = ctx.data()?;
        sqlx::query!(r"DELETE FROM products WHERE id = $1", id)
            .execute(db)
            .await
            .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))?;
        Ok(true)
    }

    async fn toggle_favorite(&self, _id: PrimaryKey) -> bool {
        todo!()
    }
}
