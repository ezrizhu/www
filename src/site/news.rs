use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn news(State(state): State<super::SiteState>) -> Markup {
    let description = "Recent news on Eric";
    let news = state.news;

    let content = html! {
        div class="news pure-g" {
            div class="pure-u-1" {
                div class="" {
                    h1 { "News" };
                    { (PreEscaped(news)) };
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/news.css";
    };
    base("", description, extra_headers, content)
}
