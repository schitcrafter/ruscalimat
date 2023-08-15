use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_poem::{GraphQLBatchRequest, GraphQLBatchResponse};
use poem::{
    handler,
    web::{Data, Html},
    IntoResponse, Result,
};
use sqlx::{Pool, Postgres};

use crate::auth::UserClaims;

mod account;
mod product;
mod purchase;
mod statistics;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    account::AccountQuery,
    product::ProductQuery,
    purchase::PurchaseQuery,
    statistics::StatisticsQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    account::AccountMutation,
    product::ProductMutation,
    purchase::PurchaseMutation,
);

#[handler]
pub async fn graphql_handler(
    GraphQLBatchRequest(mut req): GraphQLBatchRequest,
    Data(db_pool): Data<&Pool<Postgres>>,
    user_claims: Option<Data<&UserClaims>>,
) -> Result<GraphQLBatchResponse> {
    let executor = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db_pool.clone())
    .finish();

    if let Some(user_claims_data) = user_claims {
        req = req.data(user_claims_data.0.clone());
    }

    Ok(GraphQLBatchResponse(executor.execute_batch(req).await))
}

#[handler]
pub async fn graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/ruscalimat/v1/graphql")
            .finish(),
    )
}
