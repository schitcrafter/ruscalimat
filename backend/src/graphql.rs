use async_graphql::Object;

mod model;
mod account;
mod product;
mod purchase;
mod statistics;

#[derive(Default)]
pub struct GraphqlRoot;

#[Object]
impl GraphqlRoot {
    async fn test(&self) -> String {
        todo!()
    }
}
