use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn contact(State(state): State<super::SiteState>) -> Markup {
    let description = "Email: eric@ericz.me";
    let contact = state.contact.clone();

    let content = html! {
        { (PreEscaped(contact)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/post.css";
    };
    base("", description, extra_headers, content, Some(state))
}
