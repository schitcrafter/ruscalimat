use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::db::{PrimaryKey, Product, ProductType, ProductWithFavorite};

use super::extract_user_claims;

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

    async fn products_with_favorites(
        &self,
        ctx: &Context<'_>,
        product_type: Option<ProductType>,
    ) -> Result<Vec<ProductWithFavorite>> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;

        let products = sqlx::query_as!(
            ProductWithFavorite,
            r#"
            SELECT id, name, price, product_type as "product_type: ProductType",
            (
                SELECT 1 FROM favorites WHERE account_id=$1 AND product_id=products.id
            ) IS NOT NULL as "is_favorite!"
            FROM products
            WHERE $2::product_type IS NULL OR product_type = $2
            ORDER BY
            "is_favorite!" DESC,
            name ASC
            "#,
            user_claims.user_id,
            product_type as Option<ProductType>
        )
        .fetch_all(db)
        .await?;

        Ok(products)
    }

    async fn product_with_favorite(
        &self,
        ctx: &Context<'_>,
        id: PrimaryKey,
    ) -> Result<ProductWithFavorite> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;

        let product = sqlx::query_as!(
            ProductWithFavorite,
            r#"
            SELECT id, name, price, product_type as "product_type: ProductType",
            (
                SELECT 1 FROM favorites WHERE account_id=$1 AND product_id=products.id
            ) IS NOT NULL as "is_favorite!"
            FROM products
            WHERE id=$2
            "#,
            user_claims.user_id,
            id
        )
        .fetch_one(db)
        .await?;

        Ok(product)
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

    async fn toggle_favorite(&self, ctx: &Context<'_>, product_id: PrimaryKey) -> Result<bool> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;
        let user_id = &user_claims.user_id;

        let is_favorite_before: bool = sqlx::query_scalar!(
            r#"
            SELECT 1 FROM favorites WHERE account_id = $1 AND product_id = $2
            "#,
            user_id,
            product_id
        )
        .fetch_optional(db)
        .await?
        .is_some();

        if is_favorite_before {
            // remove from db
            sqlx::query!(
                r#"
            DELETE FROM favorites
            WHERE account_id = $1 AND product_id = $2
            "#,
                user_id,
                product_id
            )
            .execute(db)
            .await?;
        } else {
            // add to db
            sqlx::query!(
                r#"
            INSERT INTO favorites
            ( account_id, product_id )
            VALUES ($1, $2)
            "#,
                user_id,
                product_id
            )
            .execute(db)
            .await?;
        }

        Ok(!is_favorite_before)
    }
}
