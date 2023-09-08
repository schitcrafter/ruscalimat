use poem_openapi::{param::Path, OpenApi};

pub struct ProductPicApi;

#[OpenApi(prefix_path = "/picture/product")]
impl ProductPicApi {
    #[oai(path = "/:id", method = "post")]
    pub async fn update_product_picture(&self, Path(_id): Path<String>) {
        todo!()
    }

    #[oai(path = "/:id", method = "delete")]
    pub async fn remove_product_picture(&self, Path(_id): Path<String>) {
        todo!()
    }
}
