use poem::web::Data;
use poem_openapi::{payload::PlainText, OpenApi};

mod accountapi;
mod productapi;

pub use accountapi::AccountPicApi;
pub use productapi::ProductPicApi;
use sqlx::{Pool, Postgres};

use crate::auth::UserClaims;

pub struct PictureApi;

#[OpenApi]
impl PictureApi {
    #[oai(path = "/picture", method = "get")]
    async fn index(
        &self,
        Data(_db): Data<&Pool<Postgres>>,
        user_claims: Option<Data<&UserClaims>>,
    ) -> PlainText<String> {
        // TODO: Remove this
        if let Some(groups) = user_claims {
            let user_claims = groups.0;
            let groups = &user_claims.groups;
            return PlainText(format!("User claims: {:?}", groups));
        }
        PlainText("Hello world".to_string())
    }
}
