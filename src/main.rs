//pub mod parse;
use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use tower_http::services::ServeDir;
mod site;
mod css;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[derive(Clone)]
pub struct AppState {
    css: Vec<css::Css>
}

#[tokio::main]
async fn main() {
    println!("Startup!");

    let state = AppState {
        css: css::init()
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/assets/css/:name", get(css::get))
        .nest_service("/assets/img", get_service(ServeDir::new("./assets/img")))
        .route("/", get(site::home::home))
        .route("/contact", get(site::contact::contact))
        .route("/news", get(site::news::news))
        .with_state(state);
    

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
