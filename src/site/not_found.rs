use super::base;
use maud::{html, Markup};

pub async fn not_found() -> Markup {
    let description = "404: Not Found :-(";
    let content = html! {
        h1 { (description) }
        p { "The page you're looking for does not exist. If this was unexpected, please shoot me an email or create an issue on "
        a target="_blank" href="https://github.com/ericzty/www" { "Github" }
        "."}
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/404.css";
    };
    base("404 - Not Found", description, extra_headers, content)
}