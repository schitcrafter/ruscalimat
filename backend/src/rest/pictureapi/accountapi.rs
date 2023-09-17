use poem::error::BadRequest;
use poem::web::Data;
use poem::Result;
use poem::{error::InternalServerError, http::StatusCode};
use poem_openapi::param::Path;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use crate::auth::JwtBearerAuth;
use crate::s3::{self, full_account_picture_key, partial_picture_key};

use super::FileUpload;

pub struct AccountPicApi;

#[OpenApi(prefix_path = "/picture")]
impl AccountPicApi {
    #[oai(path = "/account/:user_id", method = "post")]
    pub async fn update_other_account_picture(
        &self,
        Path(user_id): Path<String>,
        Data(db): Data<&Pool<Postgres>>,
        JwtBearerAuth(user_claims): JwtBearerAuth,
        file_upload: FileUpload,
    ) -> Result<()> {
        if !user_claims.is_admin() {
            return Err(poem::Error::from_status(StatusCode::FORBIDDEN));
        }

        upload_account_picture(db, &user_id, file_upload).await
    }

    #[oai(path = "/account/:id", method = "delete")]
    pub async fn delete_other_account_picture(&self, Path(id): Path<String>) {
        info!("Deleting account picture with id {id}");
        todo!()
    }

    #[oai(path = "/myAccount", method = "post")]
    pub async fn update_my_account_picture(
        &self,

        Data(db): Data<&Pool<Postgres>>,
        JwtBearerAuth(user_claims): JwtBearerAuth,
        file_upload: FileUpload,
    ) -> Result<()> {
        upload_account_picture(db, &user_claims.user_id, file_upload).await
    }

    #[oai(path = "/myAccount", method = "delete")]
    pub async fn remove_my_account_picture(&self) {
        todo!()
    }
}

async fn upload_account_picture(
    db: &Pool<Postgres>,
    user_id: &str,
    file_upload: FileUpload,
) -> Result<()> {
    assert_account_exists(db, user_id).await?;

    let body = file_upload.file.into_vec().await.map_err(BadRequest)?;

    let filename = file_upload.filename.as_str();
    let part_key = partial_picture_key(filename, &user_id)?;
    let full_key = full_account_picture_key(part_key.as_str());

    s3::upload_file(full_key.as_str(), body.into())
        .await
        .map_err(InternalServerError)?;

    sqlx::query!(
        r#"
            UPDATE accounts
            SET picture = $2
            WHERE id = $1
        "#,
        user_id,
        part_key
    )
    .execute(db)
    .await
    .map_err(InternalServerError)?;

    Ok(())
}

async fn assert_account_exists(db: &Pool<Postgres>, id: &str) -> Result<()> {
    sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|_| poem::Error::from_string("Unknown id", StatusCode::BAD_REQUEST))?;
    Ok(())
}
