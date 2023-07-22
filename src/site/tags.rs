use maud::{html, Markup};
use super::base;
use super::not_found::not_found;
use crate::post::Post;
use std::collections::HashSet;
use axum::{
    extract::{Path,State},
    http::StatusCode,
};

pub async fn tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.blog.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    let content = html! {
        div class="tags" {
            h1 { "List of tags in my blog" }
            ul {
                @for tag in tags {
                    li { a href=(format!("/blog/tags/{}", tag)) { (tag) } }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/tag-index.css";
    };

    base("Tags", "List of tags in my blog.", extra_headers, content, Some(state))
}

pub async fn tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let blog = state.blog.clone();
    let posts: Vec<&Post> = blog.iter().filter(|post| post.tags.contains(&tag)).collect();

    if posts.len() == 0 {
        return not_found().await
    }

    let content = html! {
        h1 { "Posts with the tag: " (tag) }
        @for post in posts {
            div class="blog-box" {
                a href=(format!("/blog/{}", post.slug)) {
                    h2 { (post.title) }
                    @let tags_str = post.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                    p { "tags: " (tags_str) }

                    @let date_str = post.date.format("%B %d, %Y").to_string();
                    p { (date_str) " - " (post.description) }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/blog-index.css";
    };

    (StatusCode::OK, base(&tag, &format!("Posts tagged with {}.", tag), extra_headers, content, Some(state)))
}
