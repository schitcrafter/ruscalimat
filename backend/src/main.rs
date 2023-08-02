use async_graphql::{http::GraphiQLSource, Schema, EmptyMutation, EmptySubscription};
use async_graphql_poem::GraphQL;
use color_eyre::eyre::Result;
use poem::{handler, listener::TcpListener, web::Html, IntoResponse, Route, Server, get};
use poem_openapi::OpenApiService;
use tracing::info;

use crate::{rest::pictureapi::{AccountPicApi, PictureApi, ProductPicApi}, graphql::GraphqlRoot};

mod db;
mod graphql;
mod rest;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();

    let port = std::env::var("APPLICATION_PORT").unwrap_or("3000".to_owned());
    let hosted_url = format!("localhost:{port}");
    let hosted_http = format!("http://{hosted_url}");

    info!("OpenAPI explorer running at http://{hosted_url}/ruscalimat/q/docs");

    let all_endpoints = (PictureApi, AccountPicApi, ProductPicApi);

    let api_service = OpenApiService::new(all_endpoints, "Ruscalimat API", "1.0")
        .server(format!("{hosted_http}/ruscalimat/api/v1"));

    let ui = api_service.openapi_explorer();
    let spec = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();

    let schema = Schema::build(GraphqlRoot::default(), EmptyMutation, EmptySubscription)
        // .data(db_handle)
        .finish();

    let graphql_app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    let app = Route::new().nest(
        "/ruscalimat",
        Route::new()
            .nest("/api/v1", api_service)
            .nest("/q/docs", ui)
            .nest("/q/openapi", spec)
            .nest("/q/openapi.yaml", spec_yaml)
            .nest("/graphql", graphql_app),
    );
    Server::new(TcpListener::bind(hosted_url)).run(app).await?;

    Ok(())
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}
