use maud::{html, Markup};
use super::SiteState;
use crate::webring;
pub mod home;
pub mod contact;
pub mod news;
pub mod projects;
pub mod blog;
pub mod talks;
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
        "Ezri".to_string()
    } else {
        title.to_string() + " - Ezri"
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
                    meta name="author" content="Ezri Zhu";

                    link rel="apple-touch-icon" sizes="180x180" href="/assets/favicon/apple-touch-icon.png";
                    link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";
                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#19191e";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#19191e";
                    meta property="og:site_name" content="Ezri";

                    link rel="stylesheet" href="/assets/css/main.css";
                    (extra_headers)
                }
                body {
                    a id="prideflag" href="https://www.hrw.org/topic/lgbt-rights" target="_blank" { img src="/assets/img/pride.svg" alt="progressive pride flag"; }
                    div class="home-menu pure-menu pure-menu-horizontal" {
                        //a href="/" {
                        //    img class="logo" src="/assets/img/logo.svg" alt="ezri zhu";
                        //}
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
                            /*
                               li class="pure-menu-item" {
                               a class="pure-menu-link" href="/talks" {
                               "Talks"
                               }
                               }
                               */
                            li class="pure-menu-item" {
                                a class="pure-menu-link" href="/files/Tianyu_Ezri_Zhu_Resume.pdf" {
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
                    div class="banner" {
                        "I'm open to employment for summer of 2025 internships! Please contact me via the "
                            a href="/contact" { "contacts page." }
                    }
                    div class="main" {
                        (content)
                    }
                    div class="footer" {
                        @if webring_enabled {
                            p {
                                "<- " a target="_blank" href=(ring_left.url) { (ring_left.name) } " • "
                                    a target="_blank" href=(ring_random.url) { "Random" } " • "
                                    a target="_blank" href=(ring_right.url) { (ring_right.name) } " ->"
                                    br;
                                "This website is part of the "
                                    a target="_blank" href="https://github.com/Stevens-26/webring/" { "Stevens Community Webring" }
                                "."
                                    br;
                            }
                        }

                        p {
                            "⎈-657a7269"
                                br;
                            "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                br;
                            "Copyright 2018-2023 • All text here are released under "
                                a target="_blank" href="https://creativecommons.org/licenses/by/4.0/" { "(CC BY 4.0)" }
                            " • Source code "
                                a target="_blank" href="https://github.com/ezrizhu/www" { "available here" }
                            ", released under the "
                                a target="_blank" href="https://github.com/ezrizhu/www/blob/main/COPYING" { "GNU AGPLv3 license" }
                            "." 
                                br;
                            (build_info);
                        }

                        div class="badges" {
                            details {
                                summary { "Show Badges" }
                                a target="_blank" href="https://ezri.pet" {
                                    img src="/assets/img/badges/ezri.png" alt="Ezri";
                                }
                                a target="_blank" href="https://as206628.net" {
                                    img src="/assets/img/badges/ezricloud.png" alt="EzriCloud";
                                }
                                a target="_blank" href="https://www.debian.org/" {
                                    img src="/assets/img/badges/debian.gif" alt="Powered by Debian";
                                }
                                a target="_blank" href="https://kernel.org/" {
                                    img src="/assets/img/badges/xenia-now.gif" alt="xenia-now";
                                }
                                a target="_blank" href="https://neovim.io/" {
                                    img src="/assets/img/badges/vimlove.gif" alt="vim";
                                }
                                a target="_blank" href="https://tilde.town/~ezri" {
                                    img src="/assets/img/badges/tildetownpink.gif" alt="tilde.town";
                                }
                                a target="_blank" href="https://store.steampowered.com/app/400/Portal/" {
                                    img src="/assets/img/badges/aperture_labs.jpg" alt="aperture_labs";
                                }
                                a target="_blank" href="https://en.pronouns.page/terminology#nonbinary" {
                                    img src="/assets/img/badges/nb_noproblem.jpg" alt="nonbinary_noproblem";
                                }
                                a target="_blank" href="https://en.pronouns.page/terminology#pansexual" {
                                    img src="/assets/img/badges/flag-pan.png" alt="flag-pan";
                                }
                                img src="/assets/img/badges/sun.gif" alt="sun microsystems";
                                a target="_blank" href="https://jigsaw.w3.org/css-validator/" {
                                    img src="/assets/img/badges/vcss-blue.gif" alt="vcss-blue";
                                }
                                a target="_blank" href="https://validator.w3.org/feed/" {
                                    img src="/assets/img/badges/valid-atom.png" alt="valid atom";
                                }
                                a target="_blank" href="https://validator.w3.org/feed/" {
                                    img src="/assets/img/badges/valid-rss-rogers.png" alt="valid rss";
                                }
                            }
                        }

                        div class="h-card" style="display: none" {
                            a class="u-email" href="mailto:me@ezrizhu.com" {};
                            a class="u-url u-uid p-name" href="https://ezrizhu.com" { "ezri zhu" };
                            p class="p-note" { "I’m a student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps" };
                        }
                    }
                }
            }
    }
}
