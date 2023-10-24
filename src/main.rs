use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use tower_http::services::{ServeDir,ServeFile};
mod site;
mod css;
mod utils;
mod post;
mod sitemap;
mod feed;
mod webring;
mod pgp;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

enum PageType {
    Blog,
    Project,
    Talk
}

#[derive(Clone)]
pub struct SiteState {
    css: Vec<css::Css>,
    home: String,
    now: String,
    five_news: String,
    news_vec: Vec<String>,
    contact: String,
    news: String,
    projects: Vec<post::Post>,
    blog: Vec<post::Post>,
    talks: Vec<post::Post>,
    sitemap: Vec<u8>,
    webring: Vec<webring::Node>,
}


#[tokio::main]
async fn main() {
    println!("Loading state.");

    let mut state = SiteState {
        css: css::init(),
        five_news: utils::init_news(),
        news_vec: utils::read_news_to_vec(),
        contact: utils::path_to_html(&"content/contact.md"),
        news: utils::path_to_html(&"content/news.md"),
        home: utils::path_to_html(&"content/home.md"),
        now: utils::path_to_html(&"content/now.md"),
        projects: post::init(&"content/projects"),
        blog: post::init(&"content/blog"),
        talks: post::init(&"content/talks"),
        sitemap: vec![],
        webring: webring::fetch().await.unwrap(),
    };

    state.sitemap = sitemap::init(state.clone()).expect("Failed to init sitemap");

    println!("Starting webserver!");

    let app = Router::new()
        .route("/health", get(health))
        .route("/robots.txt", get_service(ServeFile::new("./assets/robots.txt")))
        .route("/assets/css/:name", get(css::get))
        .nest_service("/assets/img", get_service(ServeDir::new("./assets/img")))
        .nest_service("/assets/favicon", get_service(ServeDir::new("./assets/favicon")))
        .nest_service("/files", get_service(ServeDir::new("./assets/files")))
        .route("/", get(site::home::home))
        .route("/contact", get(site::contact::contact))
        .route("/news", get(site::news::news))
        .route("/news.atom", get(feed::news_atom::get))
        .route("/news.xml", get(feed::news_rss::get))
        .route("/now", get(site::now::now))
        .route("/projects", get(site::projects::project_index))
        .route("/projects/", get(site::projects::project_index))
        .route("/projects/:name", get(site::post::project_handler))
        .route("/projects/tags", get(site::tags::projects_tags_index))
        .route("/projects/tags/", get(site::tags::projects_tags_index))
        .route("/projects/tags/:tag", get(site::tags::projects_tags_get))
        .route("/talks", get(site::talks::talk_index))
        .route("/talks/:name", get(site::post::talk_handler))
        .route("/talks/tags", get(site::tags::talks_tags_index))
        .route("/talks/tags/", get(site::tags::talks_tags_index))
        .route("/talks/tags/:tag", get(site::tags::talks_tags_get))
        .route("/blog", get(site::blog::blog_index))
        .route("/blog/", get(site::blog::blog_index))
        .route("/blog/:name", get(site::post::blog_handler))
        .route("/blog/tags", get(site::tags::blog_tags_index))
        .route("/blog/tags/", get(site::tags::blog_tags_index))
        .route("/blog/tags/:tag", get(site::tags::blog_tags_get))
        .route("/sitemap.xml", get(sitemap::get))
        .route("/blog.xml", get(feed::blog_rss::get))
        .route("/blog.atom", get(feed::blog_atom::get))
        .route("/.well-known/openpgpkey/hu/policy", get(pgp::policy))
        .route("/.well-known/openpgpkey/hu/15asjmkpucio5m8a7xznzcxqsqigumxt", get(pgp::pubkey))
        .fallback(site::not_found::not_found)
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
