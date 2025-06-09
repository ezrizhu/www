use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn resume(State(state): State<super::SiteState>) -> Markup {
    let description = "What I've done";
    let resume = state.resume.clone();

    let content = html! {
        { (PreEscaped(resume)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
        link rel="canonical" href="https://ezrizhu.com/resume";
    };
    base("", description, extra_headers, content, Some(state))
}
