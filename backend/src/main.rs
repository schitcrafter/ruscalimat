use std::env;

use crate::rest::pictureapi::{AccountPicApi, ProductPicApi};
use color_eyre::eyre::Result;
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use poem_openapi::{ExtraHeader, OpenApiService};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

mod auth;
mod db;
mod graphql;
mod rest;
mod s3;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let auth_server_url = env::var("AUTH_SERVER_URL").unwrap();
    auth::setup(&auth_server_url).await?;
    s3::init().await;

    let port = env::var("APPLICATION_PORT").unwrap_or("3000".to_owned());
    let hosted_url = format!("localhost:{port}");
    let hosted_http = format!("http://{hosted_url}/ruscalimat");

    info!("OpenAPI explorer running at {hosted_http}/q/docs");
    info!("GraphiQL running at {hosted_http}/q/graphiql");
    info!("Login endpoint at {auth_server_url}/account/#/");

    let all_endpoints = (AccountPicApi, ProductPicApi);

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
        .around(auth::auth_middleware)
        .with(poem::middleware::Tracing);

    let app = Route::new()
        .nest(
            "/ruscalimat",
            Route::new().nest("/v1", api_routes).nest("/q", dev_paths),
        )
        .data(db_pool);

    Server::new(TcpListener::bind(hosted_url)).run(app).await?;

    Ok(())
}
