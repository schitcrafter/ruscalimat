use async_graphql::{Context, ErrorExtensions, InputObject, Object, SimpleObject};

use crate::{auth, db::Account};

use super::{extract_user_claims, types::sort::Sort};

#[derive(SimpleObject)]
struct AccountsList {
    data: Vec<Account>,
    page: u32,
    total: u32,
}

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    /// TODO: Implement `sort` parameter
    /// TODO: Implement paging
    async fn accounts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] _sort: Sort,
    ) -> async_graphql::Result<AccountsList> {
        let db = ctx.data()?;
        let accounts = sqlx::query_as!(Account, "SELECT * FROM accounts WHERE deleted_at IS NULL")
            .fetch_all(db)
            .await?;

        let total = accounts.len() as u32;

        let accounts_list = AccountsList {
            data: accounts,
            page: 1,
            total,
        };
        Ok(accounts_list)
    }

    async fn account(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Account>> {
        let db = ctx.data_unchecked();
        let account = sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = $1", id)
            .fetch_optional(db)
            .await?;
        Ok(account)
    }

    async fn my_account(&self, ctx: &Context<'_>) -> async_graphql::Result<Account> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;

        let account = sqlx::query_as!(
            Account,
            "SELECT * FROM accounts WHERE id = $1",
            user_claims.user_id
        )
        .fetch_one(db)
        .await?;
        Ok(account)
    }

    /// Returns a JWT, which can be used to authenticate later requests
    async fn pin_login(
        &self,
        ctx: &Context<'_>,
        pin_login: PinLogin,
    ) -> async_graphql::Result<String> {
        let db = ctx.data()?;
        let user = sqlx::query_as!(
            Account,
            r#"
            SELECT * FROM accounts WHERE id = $1
            "#,
            pin_login.id
        )
        .fetch_one(db)
        .await?;

        if !bcrypt::verify(pin_login.pin.to_string(), user.pin_hash.as_str())? {
            return Err(
                async_graphql::Error::new("Wrong pin").extend_with(|_, e| e.set("code", 401))
            );
        }

        let jwt = auth::create_pin_jwt(user.id.as_str())?;

        Ok(jwt)
    }

    async fn deleted_accounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Account>> {
        let db = ctx.data()?;
        let accounts = sqlx::query_as!(
            Account,
            "SELECT * FROM accounts WHERE deleted_at IS NOT NULL"
        )
        .fetch_all(db)
        .await?;
        Ok(accounts)
    }
}

#[derive(InputObject)]
struct PinLogin {
    pub id: String,
    pub pin: u16,
}

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn delete_account(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<bool> {
        let db = ctx.data()?;
        sqlx::query!("UPDATE accounts SET deleted_at = now() WHERE id = $1", id)
            .execute(db)
            .await?;
        Ok(true)
    }

    async fn signup(&self, ctx: &Context<'_>, pin: u16) -> async_graphql::Result<Account> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;
        let pin_hash = hash_pin(pin)?;
        let account = sqlx::query_as!(
            Account,
            r#"
                INSERT INTO accounts (name, id, email, pin_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_claims.name,
            user_claims.user_id,
            user_claims.email,
            pin_hash
        )
        .fetch_one(db)
        .await?;

        Ok(account)
    }

    async fn update_account(
        &self,
        ctx: &Context<'_>,
        account: Account,
    ) -> async_graphql::Result<bool> {
        let db = ctx.data()?;
        sqlx::query!(
            r#"
            UPDATE accounts
            SET name = $1, email = $2, picture = $3
            WHERE id = $4"#,
            account.name,
            account.email,
            account.picture,
            account.id
        )
        .execute(db)
        .await?;
        Ok(true)
    }

    async fn set_pin(&self, ctx: &Context<'_>, pin: u16) -> async_graphql::Result<bool> {
        let user_claims = extract_user_claims(ctx)?;
        let db = ctx.data()?;
        let pin_hash = hash_pin(pin)?;

        sqlx::query!(
            "UPDATE accounts SET pin_hash = $1 WHERE id = $2",
            pin_hash,
            user_claims.user_id
        )
        .execute(db)
        .await?;

        Ok(true)
    }
}

fn hash_pin(pin: u16) -> async_graphql::Result<String> {
    if pin > 9999 {
        return Err(async_graphql::Error::new(
            "Pin needs to be between 0 and 9999",
        ));
    }
    bcrypt::hash(pin.to_string(), bcrypt::DEFAULT_COST)
        .map_err(|err| err.extend_with(|_, e| e.set("code", 500)))
}
