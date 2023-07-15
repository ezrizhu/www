use maud::{html, Markup};
pub mod home;
pub mod contact;
pub mod utils;

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
                link rel="stylesheet" href="assets/css/nord.css";
                link rel="stylesheet" href="assets/css/pure-min.css";
                link rel="stylesheet" href="assets/css/main.css";

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
                link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🦊</text></svg>";
                meta property="og:theme-color" content="#2e3440";
                meta property="og:image" content="/assets/img/animoji.png";
                meta name="theme-color" content="#2e3440";
                (extra_headers)
            }
            body {
                div class="home-menu pure-menu pure-menu-horizontal" {
                    img class="logo" src="assets/img/logo.svg" alt="Tianyu (Eric) Zhu";
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
                            a class="pure-menu-link" href="/resume" {
                                "Resume"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/cv" {
                                "CV"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/friends" {
                                "Friends"
                            }
                        }
                        li class="pure-menu-item" {
                            a class="pure-menu-link" href="/affiliates" {
                                "Affiliates"
                            }
                        }
                    }
                }
                div class="main" {
                    (content)
                }
                div class="footer" {
                    p { 
                        p { "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present." };

                        "Copyright 2018-2023 • All text here are released under "
                        a target="_blank" href="https://creativecommons.org/licenses/by/4.0/" { "(CC BY 4.0)" }
                        " • Source code "
                        a target="_blank" href="https://github.com/ericzty/www" { "available here" }
                        ", released under the "
                        a target="_blank" href="https://github.com/ericzty/www/blob/main/COPYING" { "AGPLv3 license" }
                        "." };

                        p { (build_info)};
                    }
                }
            }
        }
    }
