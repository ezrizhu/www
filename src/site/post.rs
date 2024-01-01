use maud::{html, Markup, PreEscaped};
use super::not_found::not_found;
use crate::post::{get, Post};
use crate::SiteState;
use axum::{
    extract::{Path,State},
    http::StatusCode
};
use super::base;
use crate::PageType;

pub async fn blog_handler(Path(name): Path<String>, State(state): State<SiteState>) -> (StatusCode, Markup) {
    if let Some(blog) = get(state.blog.clone(), &name) {
        (StatusCode::OK, post(blog, state, PageType::Blog))
    } else {
        return not_found().await
    }
}

pub async fn project_handler(Path(name): Path<String>, State(state): State<SiteState>) -> (StatusCode, Markup) {
    if let Some(project) = get(state.projects.clone(), &name) {
        (StatusCode::OK, post(project, state, PageType::Project))
    } else {
        not_found().await
    }
}

pub async fn talk_handler(Path(name): Path<String>, State(state): State<SiteState>) -> (StatusCode, Markup) {
    if let Some(talk) = get(state.talks.clone(), &name) {
        (StatusCode::OK, post(talk, state, PageType::Talk))
    } else {
        not_found().await
    }
}

fn post(post: Post, state: SiteState, page_type: PageType) -> Markup {
    let tags = post.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let page_name = match page_type {
        PageType::Blog => "blog",
        PageType::Project => "projects",
        PageType::Talk => "talks",
    };

    let content = html! {
        article class="h-entry" {
            h1 class="p-name" { (post.title) };
            div class="byline" {
                p {
                    "by " a class="p-author h-card" href="https://ezrizhu.com" target="_blank" { "Ezri" }
                    @if matches!(page_type, PageType::Blog) {
                        @let date_str = post.date.format("%B %d, %Y").to_string();
                        @let date_rfc3339 = post.date.to_rfc3339();
                        " on " time class="dt-published" datetime=(date_rfc3339) { (date_str) }
                    }
                    br;
                    "tags: "
                        @for tag in tags {
                            a class="p-category" href=(format!("/{}/tags/{}", page_name, tag)) { (tag) } " "
                        }
                }
            }
            div class="e-content" {
                p { (PreEscaped(post.body)) };
            }
            a style="display: none;" class="u-url" href=(format!("https://ezrizhu.com/{}/{}", page_name, post.slug)) { "Permalink" }
        }
        hr;
        p { "If you have any questions, want to change my mind, or literally anything else, please " a href="mailto:me@ezrizhu.com" {"reach out"} "!" };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
        link rel="webmention" href="https://ezrizhu.com/webmention/accept";
        link rel="canonical" href=(format!("https://ezrizhu.com/{}/{}", page_name, post.slug));
    };
    base(&post.title, &post.description, extra_headers, content, Some(state))
}
