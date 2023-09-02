use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn news(State(state): State<super::SiteState>) -> Markup {
    let description = "Recent news on Eric";
    let news = state.news.clone();

    let content = html! {
        div class="news" {
            div class="" {
                h1 { "News" };
                p class="desc" {
                    "I have a " a href="/news.xml" { "rss feed" } " and an " a href="/news.atom" { "atom feed" } ", if you have a reader that supports them."
                }
                { (PreEscaped(news)) };
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/news.css";
        link rel="alternate" title="Eric's news" type="application/rss+xml" href="https://ericz.me/news.xml";
        link rel="alternate" title="Eric's news" type="application/atom+xml" href="https://ericz.me/news.atom";
    };
    base("", description, extra_headers, content, Some(state))
}
