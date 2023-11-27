use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn contact(State(state): State<super::SiteState>) -> Markup {
    let description = "Email: eric@ericz.me";
    let contact = state.contact.clone();

    let content = html! {
        div class="hero pure-g" {
            div class="pure-u-1 pure-u-md-1-3 headshot" {
                img src="/assets/img/eric2.webp" alt="An image of Eric Zhu sitting on a table." class="pure-img headshot";
                p class="pronouns" { "(he/they)" };
            }
            div class="pure-u-1 pure-u-md-2-3" {
                div class="contact" {
                    { (PreEscaped(contact)) };
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/contact.css";
    };
    base("", description, extra_headers, content, Some(state))
}
