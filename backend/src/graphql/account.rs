use async_graphql::{Context, Object};
use color_eyre::eyre::{self, Result};
use sqlx::{Pool, Postgres};

use crate::db::{Account, PrimaryKey};

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn accounts(&self, ctx: &Context<'_>) -> Result<Vec<Account>> {
        let db = ctx.data_unchecked();
        let accounts = sqlx::query_as!(Account, "SELECT * FROM accounts")
            .fetch_all(db)
            .await?;
        Ok(accounts)
    }

    async fn account(&self, ctx: &Context<'_>, id: PrimaryKey) -> Result<Option<Account>> {
        let db = ctx.data_unchecked();
        let account = sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = $1", id)
            .fetch_optional(db)
            .await?;
        Ok(account)
    }

    async fn my_account(&self) -> Account {
        todo!()
    }

    async fn pin_login(&self, _pin: u16) -> String {
        todo!()
    }

    async fn deleted_accounts(&self, ctx: &Context<'_>) -> Result<Vec<Account>> {
        let db = ctx.data_unchecked();
        let accounts = sqlx::query_as!(
            Account,
            "SELECT * FROM accounts WHERE deleted_at IS NOT NULL"
        )
        .fetch_all(db)
        .await?;
        Ok(accounts)
    }
}

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn delete_account(&self, ctx: &Context<'_>, id: PrimaryKey) -> Result<bool> {
        let db = ctx.data_unchecked();
        sqlx::query!("UPDATE accounts SET deleted_at = now() WHERE id = $1", id)
            .execute(db)
            .await?;
        Ok(true)
    }

    async fn signup(&self, ctx: &Context<'_>, pin: u16) -> Result<Account> {
        if pin > 9999 {
            return Err(color_eyre::eyre::eyre!(
                "Pin needs to be between 0 and 9999"
            ));
        }
        let db = ctx.data_unchecked();
        let pin_hash = hash_pin(pin);
        let account = sqlx::query_as!(
            Account,
            "INSERT INTO accounts (name, email, pin_hash) VALUES ($1, $2, $3) RETURNING *",
            "test_name",           // TODO:
            "test_email@test.com", // TODO:
            pin_hash
        )
        .fetch_one(db)
        .await?;

        Ok(account)
    }

    async fn update_account(&self, ctx: &Context<'_>, account: Account) -> Result<bool> {
        let db = ctx.data_unchecked();
        sqlx::query!(
            "UPDATE accounts SET name = $1, email = $2 WHERE id = $3",
            account.name,
            account.email,
            account.id
        )
        .execute(db)
        .await?;
        Ok(true)
    }

    async fn set_pin(&self, _pin: u16) -> bool {
        todo!()
    }
}

fn hash_pin(pin: u16) -> String {
    bcrypt::hash(pin.to_string(), bcrypt::DEFAULT_COST).unwrap()
}
