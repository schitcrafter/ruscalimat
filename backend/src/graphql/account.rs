use async_graphql::Object;

use crate::db::Account;

use super::model::PinLogin;

pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn accounts(&self) -> Vec<Account> {
        todo!()
    }

    async fn account(&self, _id: u64) -> Account {
        todo!()
    }

    async fn my_account(&self) -> Account {
        todo!()
    }

    async fn pin_login(&self, _pin: PinLogin) -> String {
        todo!()
    }

    async fn deleted_accounts(&self) -> Vec<Account> {
        todo!()
    }
}

pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn delete_account(&self, _id: u64) -> bool {
        todo!()
    }

    async fn signup(&self, _pin: PinLogin) -> Account {
        todo!()
    }

    async fn update_account(&self, _account: Account) -> bool {
        todo!()
    }

    async fn set_pin(&self, _pin: PinLogin) -> bool {
        todo!()
    }
}
