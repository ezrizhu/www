//pub mod parse;
use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use tower_http::services::ServeDir;
mod site;

// health
async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[tokio::main]
async fn main() {
    println!("Startup!");
    let app = Router::new()
        .route("/health", get(health))
        .nest_service("/assets", get_service(ServeDir::new("./assets")))
        .route("/", get(site::home::home));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
