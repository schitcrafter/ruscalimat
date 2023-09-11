use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions, MergedObject, Schema,
};
use async_graphql_poem::{GraphQLBatchRequest, GraphQLBatchResponse};
use poem::{
    handler,
    web::{
        headers::{Authorization, HeaderMapExt},
        Data, Html,
    },
    IntoResponse, Request, Result,
};
use poem_openapi::auth::Bearer;
use sqlx::{Pool, Postgres};

use crate::auth::{check_bearer, UserClaims};

mod account;
mod product;
mod purchase;
mod statistics;
mod types;

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
    rest_request: &Request,
    GraphQLBatchRequest(mut req): GraphQLBatchRequest,
    Data(db_pool): Data<&Pool<Postgres>>,
) -> Result<GraphQLBatchResponse> {
    let executor = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db_pool.clone())
    .finish();

    let bearer = rest_request
        .headers()
        .typed_get::<Authorization<poem::web::headers::authorization::Bearer>>()
        .map(|b| Bearer {
            token: b.token().to_string(),
        });

    let user_claims = if let Some(bearer) = bearer {
        Some(check_bearer(rest_request, bearer).await?)
    } else {
        None
    };

    if let Some(user_claims_data) = user_claims {
        req = req.data(user_claims_data);
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

pub fn extract_user_claims<'ctx>(
    ctx: &'ctx Context<'ctx>,
) -> async_graphql::Result<&'ctx UserClaims> {
    ctx.data()
        .map_err(|err| err.extend_with(|_, e| e.set("code", 401)))
}
