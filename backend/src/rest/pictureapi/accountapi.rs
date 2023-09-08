use poem_openapi::{param::Path, OpenApi};
use tracing::info;

pub struct AccountPicApi;

#[OpenApi(prefix_path = "/picture")]
impl AccountPicApi {
    #[oai(path = "/account/:id", method = "post")]
    pub async fn update_other_account_picture(&self, Path(id): Path<String>) {
        info!("Got id {id}");
        todo!()
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
