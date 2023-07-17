use maud::{html, Markup, PreEscaped};
use super::base;

pub fn post(title: &str, date: &str, desc: &str, body: &str) -> Markup {
    let content = html! {
        h1 { (title) };
        h3 { (date) };
        p { (PreEscaped(body)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
    };
    base(title, desc, extra_headers, content)
}
