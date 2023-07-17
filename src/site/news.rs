use maud::{html, Markup, PreEscaped};
use comrak::{markdown_to_html, ComrakOptions};
use super::{base, utils};
use std::fs;

pub async fn news() -> Markup {
    let description = "Recent news on Eric";
    let news_raw = fs::read_to_string("content/news.md").expect("Failed to read file");
    let news = markdown_to_html(&news_raw, &ComrakOptions::default());
    let news = utils::add_target_blank_to_links(news);
    let news = news.trim_end();

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
