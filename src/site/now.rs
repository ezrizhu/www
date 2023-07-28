use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn now(State(state): State<super::SiteState>) -> Markup {
    let description = "What I'm up to now";
    let now = state.now.clone();

    let content = html! {
        { (PreEscaped(now)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
    };
    base("", description, extra_headers, content, Some(state))
}
