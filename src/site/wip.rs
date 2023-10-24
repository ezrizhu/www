use super::base;
use maud::{html, Markup};
use axum::extract::State;

pub async fn _wip(State(state): State<super::SiteState>) -> Markup {
    let description = "Work in progress, please check back later.";
    let content = html! {
        h1 { (description) }
        p { "The page you're trying to visit does not exist yet, but it will be soon!" }
        p { "Please check back later :-)" }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/wip.css";
    };
    base("Work in progress", description, extra_headers, content, Some(state))
}
