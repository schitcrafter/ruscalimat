use poem_openapi::{OpenApi, param::Path};


pub struct ProductPicApi;

#[OpenApi(prefix_path = "/product")]
impl ProductPicApi {

    #[oai(path = "/:id", method = "post")]
    pub async fn update_product_picture(&self, Path(_id): Path<String>) {

    }

    #[oai(path = "/:id", method = "delete")]
    pub async fn remove_product_picture(&self, Path(_id): Path<String>) {

    }

}
