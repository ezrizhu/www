use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use tower_http::services::ServeDir;
mod site;
mod css;
mod utils;
mod projects;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[derive(Clone)]
pub struct SiteState {
    css: Vec<css::Css>,
    home: String,
    five_news: String,
    contact: String,
    news: String,
    projects: Vec<projects::Project>,
}

#[tokio::main]
async fn main() {
    println!("Loading state.");

    let mut state = SiteState {
        css: css::init(),
        five_news: utils::init_news(),
        contact: utils::path_to_html(&"content/contact.md"),
        news: utils::path_to_html(&"content/news.md"),
        projects: projects::init(),
        home: utils::path_to_html(&"content/home.md"),
    };

    println!("Starting webserver!");

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
