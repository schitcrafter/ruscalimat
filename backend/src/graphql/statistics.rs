use async_graphql::Object;

use crate::db::PurchaseCount;

#[derive(Default)]
pub struct StatisticsQuery;

#[Object]
impl StatisticsQuery {
    async fn purchase_counts_last_month(&self) -> Vec<PurchaseCount> {
        todo!()
    }
}
