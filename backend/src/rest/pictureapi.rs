use poem_openapi::{types::multipart::Upload, Multipart};

mod accountapi;
mod productapi;

pub use accountapi::AccountPicApi;
pub use productapi::ProductPicApi;

#[derive(Debug, Multipart)]
pub struct FileUpload {
    file: Upload,
    filename: String,
}
