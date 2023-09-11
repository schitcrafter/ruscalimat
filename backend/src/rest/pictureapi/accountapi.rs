use poem::error::BadRequest;
use poem::web::Data;
use poem::{error::InternalServerError, http::StatusCode};
use poem_openapi::param::Path;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use crate::auth::JwtBearerAuth;
use crate::rest::pictureapi::VALID_EXTENSIONS;
use crate::s3;

use super::FileUpload;

pub struct AccountPicApi;

#[OpenApi(prefix_path = "/picture")]
impl AccountPicApi {
    #[oai(path = "/account/:id", method = "post")]
    pub async fn update_other_account_picture(
        &self,
        Path(user_id): Path<String>,
        Data(db): Data<&Pool<Postgres>>,
        JwtBearerAuth(user_claims): JwtBearerAuth,
        file_upload: FileUpload,
    ) -> poem::Result<PlainText<String>> {
        if !user_claims.is_admin() {
            return Err(poem::Error::from_status(StatusCode::FORBIDDEN));
        }

        // make sure user exists before uploading to s3

        sqlx::query!("SELECT * FROM accounts WHERE id = $1", user_id)
            .fetch_one(db)
            .await
            .map_err(|_| poem::Error::from_string("Unknown id", StatusCode::BAD_REQUEST))?;

        let extension = std::path::Path::new(file_upload.filename.as_str()).extension();

        let extension = extension
            .and_then(|ext| ext.to_str())
            .ok_or(poem::Error::from_string(
                "Filename needs to have extension",
                StatusCode::BAD_REQUEST,
            ))?;

        if !VALID_EXTENSIONS.contains(&extension) {
            return Err(poem::Error::from_string(
                format!("Invalid extension {extension}, valid extensions: {VALID_EXTENSIONS:?}"),
                StatusCode::BAD_REQUEST,
            ));
        }

        let part_key = format!("{}_{user_id}.{extension}", Uuid::new_v4());
        let full_key = format!("account/{part_key}");

        let body = file_upload.file.into_vec().await.map_err(BadRequest)?;

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

        Ok(PlainText(part_key))
    }

    #[oai(path = "/account/:id", method = "delete")]
    pub async fn delete_other_account_picture(&self, Path(id): Path<String>) {
        info!("Deleting account picture with id {id}");
        todo!()
    }

    #[oai(path = "/myAccount", method = "post")]
    pub async fn update_my_account_picture(&self) {
        todo!()
    }

    #[oai(path = "/myAccount", method = "delete")]
    pub async fn remove_my_account_picture(&self) {
        todo!()
    }
}
