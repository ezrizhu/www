use maud::{html, Markup, PreEscaped};
use axum::extract::State;
use super::base;
use crate::post;

pub async fn home(State(state): State<super::SiteState>) -> Markup {
    let description = "Student interested in software development, computer networking, managing infrastructure at scale, cybersecurity, and DevOps";

    let bio = state.home.clone();
    let news = state.five_news.clone();

    let projects = vec!["ezricloud", "try", "eve"];
    let blogs = state.blog[0..7].to_vec();

    let content = html! {
        div class="hero pure-g" {
            div class="pure-u-1 pure-u-md-1-3" {
                img src="/assets/img/ezripicrew.webp" alt="Picrew from https://picrew.me/en/image_maker/1272810" class="pure-img headshot";
                p class="pronouns" { "(they/any)" };
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
        link rel="stylesheet" href="/assets/css/home.css";
        link rel="authorization_endpoint" href="https://indieauth.com/auth";
        link rel="me" href="mailto:me@ezrizhu.com";
        link rel="canonical" href="https://ezrizhu.com/";
        link rel="alternate" title="Ezri's Blog" type="application/rss+xml" href="https://ezrizhu.com/blog.xml";
        link rel="alternate" title="Ezri's Blog" type="application/atom+xml" href="https://ezrizhu.com/blog.atom";
        link rel="alternate" title="Ezri's news" type="application/rss+xml" href="https://ezrizhu.com/news.xml";
        link rel="alternate" title="Ezri's news" type="application/atom+xml" href="https://ezrizhu.com/news.atom";
    };
    base("", description, extra_headers, content, Some(state.clone()))
}
