use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use tower_http::services::{ServeDir,ServeFile};
mod site;
mod css;
mod utils;
mod projects;
mod blog;
mod post;
mod sitemap;
mod google;

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
    projects: Vec<post::Post>,
    blog: Vec<post::Post>,
    sitemap: Vec<u8>,
}

#[tokio::main]
async fn main() {
    println!("Loading state.");

    let mut state = SiteState {
        css: css::init(),
        five_news: utils::init_news(),
        contact: utils::path_to_html(&"content/contact.md"),
        news: utils::path_to_html(&"content/news.md"),
        home: utils::path_to_html(&"content/home.md"),
        projects: projects::init(),
        blog: blog::init(),
        sitemap: vec![],
    };

    state.sitemap = sitemap::init(state.clone()).expect("Failed to init sitemap");

    google::ping().await.expect("Failed to ping Google");

    println!("Starting webserver!");

    let app = Router::new()
        .route("/health", get(health))
        .route("/robots.txt", get_service(ServeFile::new("./assets/robots.txt")))
        .route("/assets/css/:name", get(css::get))
        .nest_service("/assets/img", get_service(ServeDir::new("./assets/img")))
        .nest_service("/files", get_service(ServeDir::new("./assets/files")))
        .route("/", get(site::home::home))
        .route("/contact", get(site::contact::contact))
        .route("/news", get(site::news::news))
        .route("/projects", get(site::projects::project_index))
        .route("/projects/", get(site::projects::project_index))
        .route("/projects/:name", get(site::projects::project_handler))
        .route("/blog", get(site::blog::blog_index))
        .route("/blog/", get(site::blog::blog_index))
        .route("/blog/:name", get(site::blog::blog_handler))
        .route("/friends", get(site::wip::wip))
        .route("/affiliates", get(site::wip::wip))
        .route("/resume", get(site::wip::wip))
        .route("/cv", get(site::wip::wip))
        .route("/sitemap.xml", get(sitemap::get))
        .fallback(site::not_found::not_found)
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
