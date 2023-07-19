use maud::{html, Markup, PreEscaped};
use super::not_found::not_found;
use crate::post::{get, Post};
use axum::{
    extract::{Path,State},
    http::StatusCode
};
use super::base;

pub async fn blog_handler(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {
    if let Some(blog) = get(state.blog, &name) {
        (StatusCode::OK, post(blog, true))
    } else {
        not_found().await
    }
}

pub async fn project_handler(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {
    if let Some(project) = get(state.projects, &name) {
        (StatusCode::OK, post(project, false))
    } else {
        not_found().await
    }
}

fn post(post: Post, show_date: bool) -> Markup {
    let content = html! {
        h1 { (post.title) };
        @if show_date {
            @let date_str = post.date.format("%B %d, %Y").to_string();
            h3 { (date_str) };
        }
        p { (PreEscaped(post.body)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
    };
    base(&post.title, &post.description, extra_headers, content)
}
