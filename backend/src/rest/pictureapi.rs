use poem::web::Data;
use poem_openapi::{OpenApi, payload::PlainText};

mod accountapi;
mod productapi;

pub use accountapi::AccountPicApi;
pub use productapi::ProductPicApi;
use sqlx::{Postgres, Pool};

pub struct PictureApi;

#[OpenApi]
impl PictureApi {
    #[oai(path = "/picture", method = "get")]
    async fn index(&self, Data(db): Data<&Pool<Postgres>>) -> PlainText<&'static str> {
        PlainText("Hello World, #2")
    }
}
