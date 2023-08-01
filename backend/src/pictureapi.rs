use poem_openapi::{OpenApi, payload::PlainText};

pub mod accountapi;
pub mod productapi;

pub struct PictureApi;

#[OpenApi]
impl PictureApi {
    #[oai(path = "/picture", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World, #2")
    }
}
