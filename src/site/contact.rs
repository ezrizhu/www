use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn contact(State(state): State<super::SiteState>) -> Markup {
    let description = "Contact: eric@ericz.me\nPhone (Recorded, Toll-free) +1 8772066280";
    let contact = state.contact;

    let content = html! {
        div class="hero pure-g" {
            div class="pure-u-1 pure-u-md-1-3 headshot" {
                img src="/assets/img/eric2.webp" alt="An image of Eric Zhu sitting on a table." class="pure-img headshot";
                p class="pronouns" { "(he/him)" };
            }
            div class="pure-u-1 pure-u-md-2-3 contact" {
                div class="" {
                    { (PreEscaped(contact)) };
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/contact.css";
    };
    base("", description, extra_headers, content)
}
