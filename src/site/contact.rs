use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn contact(State(state): State<super::SiteState>) -> Markup {
    let description = "Email: me@ezrizhu.com";
    let contact = state.contact.clone();

    let content = html! {
        { (PreEscaped(contact)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
        link rel="canonical" href="https://ezrizhu.com/contact";
    };
    base("", description, extra_headers, content, Some(state))
}
