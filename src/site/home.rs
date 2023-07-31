use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;
use crate::post;

pub async fn home(State(state): State<super::SiteState>) -> Markup {
    let description = "Student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps";

    let bio = state.home.clone();
    let news = state.five_news.clone();

    let projects = vec!["ericnet", "try", "eve"];
    let blogs = state.blog[0..8].to_vec();

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
                        @for blog in blogs {
                            @let date_str = blog.date.format("%B %d, %Y").to_string();
                            li { p { (date_str) " - " a href=(format!("/blog/{}", blog.slug))  { (blog.title) } } };
                        }
                    }
                }
            }
        }
        p class="separator" { strong { "Featured Projects" } };
        div class="featured-projects pure-g" {
            @for project in projects {
                @let project = post::get(state.projects.clone(), project).unwrap();
                div class="pure-u-1 pure-u-md-1-3" {
                div class="home-box" {
                    a href=(format!("/projects/{}", project.slug)) {
                        h2 {
                            (project.title)
                        }
                    }
                    p {
                        (project.description)
                    }
                }
            }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="assets/css/home.css";
        link rel="me" href="mailto:eric@ericz.me";
    };
    base("", description, extra_headers, content, Some(state.clone()))
}
