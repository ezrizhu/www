use maud::{html, Markup};
use super::base;
use super::not_found::not_found;
use crate::post::Post;
use std::collections::HashSet;
use axum::{
    extract::{Path,State},
    http::StatusCode,
};

pub async fn blog_tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.blog.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    let content = html! {
        div class="box" {
            h1 { "List of tags in my blog" }
            ul {
                @for tag in tags {
                    li { a href=(format!("/blog/tags/{}", tag)) { (tag) } }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };

    base("Tags", "List of tags in my blog.", extra_headers, content, Some(state))
}

pub async fn projects_tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.projects.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    let content = html! {
        div class="box" {
            h1 { "List of tags in my projects" }
            ul {
                @for tag in tags {
                    li { a href=(format!("/projects/tags/{}", tag)) { (tag) } }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };

    base("Tags", "List of tags in my projects.", extra_headers, content, Some(state))
}

pub async fn blog_tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let blog = state.blog.clone();
    let blog: Vec<&Post> = blog.iter().filter(|post| post.tags.contains(&tag)).collect();

    if blog.len() == 0 {
        return not_found().await
    }

    let content = html! {
        div class="box" {
            h1 { "Posts with the tag: " (tag) }
            ul {
                @for blog in blog {
                    li {
                        p {
                            @let date_str = blog.date.format("%B %d, %Y").to_string();
                            (date_str) ": "
                                a href=(format!("/blog/{}", blog.slug)) {
                                    { (blog.title) }
                                }
                            " - "
                                (blog.description)
                        }
                    }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };

    (StatusCode::OK, base(&tag, &format!("Posts tagged with {}.", tag), extra_headers, content, Some(state)))
}

pub async fn projects_tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let projects = state.projects.clone();
    let projects: Vec<&Post> = projects.iter().filter(|post| post.tags.contains(&tag)).collect();

    if projects.len() == 0 {
        return not_found().await
    }

    let content = html! {
        div class="box" {
            h1 { "Posts with the tag: " (tag) }
            ul {
                @for project in projects {
                    li {
                        p {
                            a href=(format!("/projects/{}", project.slug)) {
                                { (project.title) }
                            }
                            " - "
                                (project.description)
                        }
                    }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };

    (StatusCode::OK, base(&tag, &format!("Posts tagged with {}.", tag), extra_headers, content, Some(state)))
}
