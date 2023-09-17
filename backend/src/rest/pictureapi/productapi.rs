use poem::{
    error::{BadRequest, InternalServerError},
    web::Data,
    Result,
};
use poem_openapi::{param::Path, OpenApi};
use reqwest::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{
    auth::JwtBearerAuth,
    db::PrimaryKey,
    s3::{self, full_product_picture_key, partial_picture_key},
};

use super::FileUpload;

pub struct ProductPicApi;

#[OpenApi(prefix_path = "/picture/product")]
impl ProductPicApi {
    #[oai(path = "/:id", method = "post")]
    pub async fn update_product_picture(
        &self,
        Path(product_id): Path<PrimaryKey>,
        Data(db): Data<&Pool<Postgres>>,
        JwtBearerAuth(user_claims): JwtBearerAuth,
        file_upload: FileUpload,
    ) -> Result<()> {
        if !user_claims.is_admin() {
            return Err(poem::Error::from_status(StatusCode::FORBIDDEN));
        }

        info!("Uploading product picture for {product_id}");

        assert_product_exists(db, product_id).await?;

        let body = file_upload.file.into_vec().await.map_err(BadRequest)?;

        let filename = file_upload.filename.as_str();
        let part_key = partial_picture_key(filename, &product_id.to_string())?;
        let full_key = full_product_picture_key(part_key.as_str());

        s3::upload_file(full_key.as_str(), body.into())
            .await
            .map_err(InternalServerError)?;

        sqlx::query!(
            r#"
            UPDATE products
            SET picture = $2
            WHERE id = $1
        "#,
            product_id,
            part_key
        )
        .execute(db)
        .await
        .map_err(InternalServerError)?;

        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    pub async fn remove_product_picture(
        &self,
        Path(product_id): Path<PrimaryKey>,
        Data(db): Data<&Pool<Postgres>>,
        JwtBearerAuth(user_claims): JwtBearerAuth,
    ) -> Result<()> {
        if !user_claims.is_admin() {
            return Err(poem::Error::from_status(StatusCode::FORBIDDEN));
        }
        info!("Deleting product picture for {product_id}");

        let account = sqlx::query!(
            r#"
        SELECT picture FROM products WHERE id = $1
    "#,
            product_id
        )
        .fetch_one(db)
        .await
        .map_err(|_| poem::Error::from_string("Unknown id", StatusCode::BAD_REQUEST))?;

        let picture = account.picture.ok_or(poem::Error::from_string(
            "Product has no picture",
            StatusCode::BAD_REQUEST,
        ))?;

        let full_key = full_product_picture_key(picture.as_str());

        s3::delete_file(full_key.as_str())
            .await
            .map_err(InternalServerError)?;

        Ok(())
    }
}

async fn assert_product_exists(db: &Pool<Postgres>, product_id: PrimaryKey) -> Result<()> {
    sqlx::query!("SELECT id FROM products WHERE id = $1", product_id)
        .fetch_one(db)
        .await
        .map_err(|_| poem::Error::from_string("Unknown id", StatusCode::BAD_REQUEST))?;
    Ok(())
}
