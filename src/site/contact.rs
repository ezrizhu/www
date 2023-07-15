use maud::{html, Markup, PreEscaped};
use comrak::{markdown_to_html, ComrakOptions};
use super::{base, utils};
use std::fs;

pub async fn contact() -> Markup {
    let description = "Contact: eric@ericz.me";
    let contact_raw = fs::read_to_string("content/contact.md").expect("Failed to read file");
    let contact = markdown_to_html(&contact_raw, &ComrakOptions::default());
    let contact = utils::add_target_blank_to_links(contact);
    let contact = contact.trim_end();

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
        link rel="stylesheet" href="assets/css/grids-responsive-min.css";
    };
    base("", description, extra_headers, content)
}
