use async_graphql::{Object, Result};

use crate::db::{AccountPurchaseCount, ProductPurchaseCount};

#[derive(Default)]
pub struct StatisticsQuery;

#[Object]
impl StatisticsQuery {
    async fn purchase_counts_last_month(&self) -> Result<Vec<ProductPurchaseCount>> {
        todo!()
    }

    async fn leaderboard(&self) -> Result<Vec<AccountPurchaseCount>> {
        todo!()
    }
}
