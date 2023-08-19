use std::env;

use color_eyre::eyre::Result;
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::rest::pictureapi::{AccountPicApi, PictureApi, ProductPicApi};

mod auth;
mod db;
mod graphql;
mod rest;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    auth::setup(&env::var("AUTH_SERVER_URL").unwrap()).await?;

    let port = env::var("APPLICATION_PORT").unwrap_or("3000".to_owned());
    let hosted_url = format!("localhost:{port}");
    let hosted_http = format!("http://{hosted_url}/ruscalimat");

    info!("OpenAPI explorer running at {hosted_http}/q/docs");
    info!("GraphiQL running at {hosted_http}/q/graphiql");

    let all_endpoints = (PictureApi, AccountPicApi, ProductPicApi);

    let api_service = OpenApiService::new(all_endpoints, "Ruscalimat API", "1.0")
        .server(format!("{hosted_http}/v1/rest"));

    let ui = api_service.openapi_explorer();
    let openapi_spec = api_service.spec_endpoint();
    let openapi_spec_yaml = api_service.spec_endpoint_yaml();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;
    sqlx::migrate!().run(&db_pool).await?;

    let dev_paths = Route::new()
        .nest("/graphiql", get(graphql::graphiql_handler))
        .nest("/docs", ui)
        .nest("/openapi", openapi_spec)
        .nest("/openapi.yaml", openapi_spec_yaml);

    let api_routes = Route::new()
        .nest("/rest", api_service)
        .at("/graphql", post(graphql::graphql_handler))
        .with(Cors::new())
        .around(auth::auth_middleware);

    let app = Route::new()
        .nest(
            "/ruscalimat",
            Route::new().nest("/v1", api_routes).nest("/q", dev_paths),
        )
        .data(db_pool);

    Server::new(TcpListener::bind(hosted_url)).run(app).await?;

    Ok(())
}
