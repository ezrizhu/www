use maud::{html, Markup};
use super::not_found::not_found;
use crate::post::get_all;
use super::post::post;
use super::base;
use axum::{
    extract::{Path,State},
    http::StatusCode
};

pub async fn blog_handler(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {
    if let Some(blog) = get_all(state.blog, &name) {
        (StatusCode::OK, post(&blog.title, &blog.date, &blog.description, &blog.body))
    } else {
        (StatusCode::NOT_FOUND, not_found().await)
    }
}

pub async fn blog_index(State(state): State<super::SiteState>) -> Markup {
    let blog = state.blog;
    let content = html! {
        h1 { "Blog" };
        @for blog in blog {
            div class="blog-box" {
                a href=(format!("/blog/{}", blog.slug)) { 
                    h2 { (blog.title) }
                    p { (blog.date) " - " (blog.description) }
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/blog-index.css";
    };
    base("Blog", "My blog.", extra_headers, content)
}
