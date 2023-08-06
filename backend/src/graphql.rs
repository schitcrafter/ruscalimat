use async_graphql::{http::GraphiQLSource, MergedObject};
use poem::{handler, web::Html, IntoResponse};

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
pub async fn graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/ruscalimat/graphql")
            .finish(),
    )
}
