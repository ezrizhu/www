use maud::{html, Markup};
use super::SiteState;
use crate::webring;
pub mod home;
pub mod contact;
pub mod news;
pub mod projects;
pub mod blog;
pub mod not_found;
pub mod wip;
pub mod post;
pub mod tags;
pub mod now;

pub fn base(title: &str, description: &str, extra_headers: Markup, content: Markup, state: Option<SiteState>) -> Markup {
    let build_info = format!("Built on: {} • Ref: {} • Commit: {} • CT: {}",
                             std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("CT").unwrap_or_else(|_| String::from("Unknown")),
                             );

    let title: String = if title.is_empty() {
        "Tianyu (Eric) Zhu".to_string()
    } else {
        title.to_string() + " - Tianyu (Eric) Zhu"
    };

    let mut webring_enabled = false;
    let mut ring_left = webring::Node{id: String::new(),name: String::new(),url: String::new()};
    let mut ring_right = webring::Node{id: String::new(),name: String::new(),url: String::new()};
    let mut ring_random = webring::Node{id: String::new(),name: String::new(),url: String::new()};

    if let Some(state) = state {
        (ring_left, ring_right) = webring::get_neighbors(state.webring.clone());
        ring_random = webring::get_random(state.webring);
        webring_enabled = true;
    };

    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";

                    title { (title) };
                    meta name="description" content=(description);
                    meta name="author" content="Tianyu (Eric) Zhu";

                    link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                    link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#2e3440";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#2e3440";
                    meta property="og:site_name" content="Eric's Blog";

                    link rel="stylesheet" href="/assets/css/main.css";
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
                                a class="pure-menu-link" href="/now" {
                                    "Now"
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
                                a class="pure-menu-link" href="/talks" {
                                    "Talks"
                                }
                            }
                            li class="pure-menu-item" {
                                a class="pure-menu-link" href="/files/Tianyu_Zhu_Resume.pdf" {
                                    "Resume"
                                }
                            }
                            li class="pure-menu-item" {
                                a class="pure-menu-link" href="/contact" {
                                    "Contact"
                                }
                            }
                        }
                    }
                    /*
                    div class="banner" {
                        "I'm open to employment for summer of 2024 internships! Please contact me via the "
                        a href="/contact" { "contacts page." }
                    }
                    */
                    div class="main" {
                        (content)
                    }
                    div class="footer" {
                        p {

                            @if webring_enabled {
                                 "<- " a target="_blank" href=(ring_left.url) { (ring_left.name) } " • "
                                 a target="_blank" href=(ring_random.url) { "Random" } " • "
                                 a target="_blank" href=(ring_right.url) { (ring_right.name) } " ->"
                                 br;
                                "This website is part of the "
                                a target="_blank" href="https://github.com/Stevens-26/webring/" { "Stevens Community Webring" }
                                "."
                                 br;
                                 br;
                                 a target="_blank" href="https://xn--sr8hvo.ws/previous"{"<--"}
                                 " • This website is also part of the "
                                 a target="_blank" href="https://xn--sr8hvo.ws" { "IndieWeb Webring 🕸💍"} " • "
                                 a target="_blank" href="https://xn--sr8hvo.ws/next" { "-->" }
                                 br;
                                 br;
                            }

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
                    div class="h-card" style="display: none" {
                        link class="u-email" rel="me" href="mailto:eric@ericz.me";
                        img class="u-photo" src="https://ericz.me/assets/img/eric1.webp" alt="my face";
                        a class="u-url u-uid p-name" href="https://ericz.me" { "Tianyu (Eric) Zhu" };
                        p class="p-note" { "I’m a student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps" };
                    }
                }
            }
    }
}
