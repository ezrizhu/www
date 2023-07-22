use maud::{html, Markup, PreEscaped};
use super::not_found::not_found;
use crate::post::{get, Post};
use crate::SiteState;
use axum::{
    extract::{Path,State},
    http::StatusCode
};
use super::base;

pub async fn blog_handler(Path(name): Path<String>, State(state): State<SiteState>) -> (StatusCode, Markup) {
    if let Some(blog) = get(state.blog.clone(), &name) {
        (StatusCode::OK, post(blog, state, true))
    } else {
        return not_found().await
    }
}

pub async fn project_handler(Path(name): Path<String>, State(state): State<SiteState>) -> (StatusCode, Markup) {
    if let Some(project) = get(state.projects.clone(), &name) {
        (StatusCode::OK, post(project, state, false))
    } else {
        not_found().await
    }
}

fn post(post: Post, state: SiteState, show_date: bool) -> Markup {
    let content = html! {
        h1 { (post.title) };
        div class="byline" {
            p { "by "
                a href="/" target="_blank" { "Eric" }
            @if show_date {
                @let date_str = post.date.format("%B %d, %Y").to_string();
                @let date_rfc3339 = post.date.to_rfc3339();
                ", on " time datetime=(date_rfc3339) { (date_str) }
            }
            @let tags_str = post.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
            br;
            "tags: " (tags_str)
            }
        }
        p { (PreEscaped(post.body)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
    };
    base(&post.title, &post.description, extra_headers, content, Some(state))
}
