use async_graphql::Object;

pub mod account;

#[derive(Default)]
pub struct GraphqlRoot;

#[Object]
impl GraphqlRoot {
    async fn test(&self) -> String {
        todo!()
    }
}
