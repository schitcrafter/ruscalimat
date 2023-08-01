use poem::{Route, listener::TcpListener, Server};
use poem_openapi::{OpenApi, payload::PlainText, OpenApiService};
use tracing::info;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World, #2")
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    info!("Rapidoc (OpenAPI site) running at {}", "http://127.0.0.1:3000/docs");
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://127.0.0.1:3000");
    let ui = api_service.rapidoc();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
