use maud::{html, Markup};
use super::SiteState;
pub mod home;
pub mod contact;
pub mod news;
pub mod projects;
pub mod blog;
pub mod not_found;
pub mod wip;
pub mod post;
pub mod tags;

pub fn base(title: &str, description: &str, extra_headers: Markup, content: Markup) -> Markup {
    let build_info = format!("Built on: {} • Ref: {} • Commit: {} • CT: {}",
                             std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("CT").unwrap_or_else(|_| String::from("Unknown")),
                             );
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

                title {
                    @if title.is_empty() {
                        "Tianyu (Eric) Zhu"
                    } @else {
                        (title) " - Tianyu (Eric) Zhu"
                    }
                }
                meta name="description" content=(description);
                meta name="author" content="Tianyu (Eric) Zhu";
                meta property="og:type" content="website";
                meta property="og:title" content="Tianyu (Eric) Zhu";
                meta property="og:description" content=(description);

                link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                link rel="manifest" href="/assets/favicon/site.webmanifest";

                link rel="stylesheet" href="/assets/css/main.css";
                meta property="og:theme-color" content="#2e3440";
                meta property="og:image" content="/assets/img/animoji.png";
                meta name="theme-color" content="#2e3440";
                (extra_headers)
            }
            body {
                div class="home-menu pure-menu pure-menu-horizontal" {
                    a href="/" {
                        img class="logo" src="/assets/img/logo.svg" alt="Tianyu (Eric) Zhu";
                    }
                    ul class="pure-menu-list" {
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/" {
                                "Home"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/news" {
                                "News"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/blog" {
                                "Blog"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/projects" {
                                "Projects"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/contact" {
                                "Contact"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/files/Tianyu_Zhu_Resume.pdf" {
                                "Resume"
                            }
                        }
                    }
                }
                div class="main" {
                    (content)
                }
                div class="footer" {
                    p {
                        "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                        br;
                        "Copyright 2018-2023 • All text here are released under "
                        a target="_blank" href="https://creativecommons.org/licenses/by/4.0/" { "(CC BY 4.0)" }
                        " • Source code "
                        a target="_blank" href="https://github.com/ericzty/www" { "available here" }
                        ", released under the "
                        a target="_blank" href="https://github.com/ericzty/www/blob/main/COPYING" { "GNU AGPLv3 license" }
                        "." 
                        br;
                        (build_info);
                    };
                }
            }
        }
    }
}
