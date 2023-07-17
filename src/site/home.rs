use maud::{html, Markup, PreEscaped};
use comrak::{markdown_to_html, ComrakOptions};
use super::{base, utils};
use std::fs;

pub async fn home() -> Markup {
    let description = "Student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps";

    let bio_raw = fs::read_to_string("content/home.md").expect("Failed to read file");
    let bio = markdown_to_html(&bio_raw, &ComrakOptions::default());
    let bio = utils::add_target_blank_to_links(bio);
    let bio = bio.trim_end();

    let news_raw = utils::read_first_five_news();
    let news = markdown_to_html(&news_raw, &ComrakOptions::default());
    let news = utils::add_target_blank_to_links(news);
    let news = news.trim_end();

    let content = html! {
        div class="hero pure-g" {
            div class="pure-u-1 pure-u-md-1-3" {
                img src="/assets/img/eric1.webp" alt="An image of Eric Zhu wearing a cow hat." class="pure-img headshot";
                p class="pronouns" { "(he/him)" };
            }
            div class="pure-u-1 pure-u-md-2-3" {
                div class="biography" {
                    { (PreEscaped(bio)) };
                }
            }
        }
        div class="pure-g recents" {
            div class="pure-u-1 pure-u-md-1-2" {
                p class="separator" { strong { "Recent News" } };
                div class="recent-list" {
                    { (PreEscaped(news)) };
                }
            }
            div class="pure-u-1 pure-u-md-1-2" {
                p class="separator" { strong { "Recent Blogs" } };
                div class="recent-list" {
                    ul {
                        li { "Date: Blog" };
                        li { "Date: Blog" };
                        li { "Date: Blog" };
                        li { "Date: Blog" };
                        li { "Date: Blog" };
                    }
                }
            }
        }
        p class="separator" { strong { "Featured Projects" } };
        div class="featured-projects pure-g" {
            div class="pure-u-1 pure-u-md-1-3" {
                div class="home-box" {
                    div class="box-title" {
                        h2 { "EricNet" }
                    }
                    div class="box-desc" {
                        p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-3" {
                div class="home-box" {
                    div class="box-title" {
                        h2 { "EVE Virtual Environment" }
                    }
                    div class="box-desc" {
                        p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-3" {
                div class="home-box" {
                    div class="box-title" {
                        h2 { "Try" }
                    }
                    div class="box-desc" {
                        p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                    }
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/home.css";
    };
    base("", description, extra_headers, content)
}
