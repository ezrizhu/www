use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;

pub async fn home(State(state): State<super::SiteState>) -> Markup {
    let description = "Student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps";

    let bio = state.home;
    let news = state.five_news;

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
