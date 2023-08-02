use async_graphql::Object;

use crate::db::account::Account;

struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn accounts(&self) -> Vec<Account> {
        todo!()
    }

    async fn account(&self, _id: u64) -> Account {
        todo!()
    }
}
