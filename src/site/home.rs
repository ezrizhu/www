use maud::{html, Markup, PreEscaped};
use comrak::{markdown_to_html, ComrakOptions};
use super::{base, utils};

pub async fn home() -> Markup {
    let description = "Student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps";

    let intro_raw = "# Hey, I'm a student interested in software **development**, computer **networking**, managing **infrastructure** at **scale**, **cybersecurity**, and **DevOps**.";
    let intro = markdown_to_html(intro_raw, &ComrakOptions::default());
    let intro = intro.trim_end();

    let desc_raw = "I am a raising sophomore undergraduate student at [Stevens Institute of Technology](https://www.stevens.edu/school-engineering-science/departments/computer-science) pursuing a Bachelor of Science degree in Computer Science. I'm currently working with [Michael Greenberg](https://greenberg.science/) and the rest of the [PaSH Team](https://binpa.sh/) on [a speculative execution engine for the shell](https://sigops.org/s/conferences/hotos/2023/papers/liargkovas.pdf).";
    let desc = markdown_to_html(desc_raw, &ComrakOptions::default());
    let desc = utils::add_target_blank_to_links(desc);
    let desc = desc.trim_end();

    let links_raw = "[Github](https://github.com/ericzty) • [LinkedIn](https://linkedin.com/in/tianyu-zhu-577356250) • [Fedi](https://uwu.social/@eric) • [Bsky](https://bsky.app/profile/ericz.me) • [Twitter](https://twitter.com/ericzty) • [Steam](https://steamcommunity.com/id/finnekit) • [Email](mailto:eric@ericz.me)";
    let links = markdown_to_html(links_raw, &ComrakOptions::default());
    let links = utils::add_target_blank_to_links(links);
    let links = links.trim_end();

    let content = html! {
        div class="hero pure-g" {
            div class="pure-u-1 pure-u-md-1-3" {
                img src="/assets/img/eric1.webp" alt="An image of Eric Zhu wearing a cow hat." class="pure-img headshot";
                p class="pronouns" { "(he/him)" };
            }
            div class="pure-u-1 pure-u-md-2-3" {
                div class="biography" {
                    div class="intro" { (PreEscaped(intro)) };
                    div class="description" { (PreEscaped(desc)) };
                    div class="links" { (PreEscaped(links)) };
                }
            }
        }
        div class="pure-g recents" {
            div class="pure-u-1 pure-u-md-1-2" {
                p class="separator" { strong { "Recent News" } };
                div class="recent-list" {
                    ul {
                        li { "Date: News" };
                        li { "Date: News" };
                        li { "Date: News" };
                        li { "Date: News" };
                        li { "Date: News" };
                    }
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
        link rel="stylesheet" href="assets/css/grids-responsive-min.css";
    };
    base("", description, extra_headers, content)
}
