use poem_openapi::{OpenApi, payload::PlainText};

mod accountapi;
mod productapi;

pub use accountapi::AccountPicApi;
pub use productapi::ProductPicApi;

pub struct PictureApi;

#[OpenApi]
impl PictureApi {
    #[oai(path = "/picture", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World, #2")
    }
}
