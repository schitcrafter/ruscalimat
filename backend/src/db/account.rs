use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Account {
    name: String,
}
