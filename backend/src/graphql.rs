use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_poem::{GraphQLBatchRequest, GraphQLBatchResponse};
use poem::{
    handler,
    web::{Data, Html},
    FromRequest, IntoResponse, Request, Result,
};
use sqlx::{Pool, Postgres};

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
    GraphQLBatchRequest(req): GraphQLBatchRequest,
    Data(db_pool): Data<&Pool<Postgres>>,
) -> Result<GraphQLBatchResponse> {
    let executor = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db_pool.clone())
    .finish();

    let user = todo!("extract user from request");

    let req = req.data(user);

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
