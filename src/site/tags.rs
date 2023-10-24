use maud::{html, Markup};
use super::base;
use super::not_found::not_found;
use crate::post::Post;
use crate::PageType;
use std::collections::HashSet;
use axum::{
    extract::{Path,State},
    http::StatusCode,
};

fn tags_index(State(state): State<super::SiteState>, page_type: PageType, tags: Vec<String>) -> Markup {
    let page_name = match page_type {
        PageType::Blog => "blog",
        PageType::Project => "projects",
        PageType::Talk => "talks",
    };

    let content = html! {
        div class="box" {
            h1 { "List of tags in my " (page_name) }
            ul {
                @for tag in tags {
                    li { a href=(format!("/{}/tags/{}", page_name, tag)) { (tag) } }
                }
            }
        }
    };

    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };

    base("Tags", &format!("List of tags in my {}.", page_name), extra_headers, content, Some(state))
}

pub async fn blog_tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.blog.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    tags_index(State(state), PageType::Blog, tags)
}

pub async fn projects_tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.projects.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    tags_index(State(state), PageType::Project, tags)
}

pub async fn talks_tags_index(State(state): State<super::SiteState>) -> Markup {

    let tags: HashSet<String> = state.talks.clone().iter().flat_map(|post| post.tags.iter().map(|tag| tag.to_string())).collect();
    let mut tags = Vec::from_iter(tags);
    tags.sort();

    tags_index(State(state), PageType::Talk, tags)
}

async fn tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>, posts: Vec<&Post>, page_type: PageType) -> (StatusCode, Markup) {
    let page_name = match page_type {
        PageType::Blog => "blog",
        PageType::Project => "projects",
        PageType::Talk => "talks",
    };

    if posts.len() == 0 {
        return not_found().await
    }

    let content = html! {
        div class="box" {
            h1 { "Posts with the tag: " (tag) }
            ul {
                @for post in posts {
                    li {
                        p {
                            @let date_str = post.date.format("%B %d, %Y").to_string();
                            (date_str) ": "
                                a href=(format!("/{}/{}", page_name, post.slug)) {
                                    { (post.title) }
                                }
                            " - "
                                (post.description)
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

pub async fn blog_tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let blog = state.blog.clone();
    let blog: Vec<&Post> = blog.iter().filter(|post| post.tags.contains(&tag)).collect();

    tags_get(Path(tag), State(state), blog, PageType::Blog).await
}

pub async fn projects_tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let projects = state.projects.clone();
    let projects: Vec<&Post> = projects.iter().filter(|post| post.tags.contains(&tag)).collect();

    tags_get(Path(tag), State(state), projects, PageType::Project).await
}

pub async fn talks_tags_get(Path(tag): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {

    let talks = state.talks.clone();
    let talks: Vec<&Post> = talks.iter().filter(|post| post.tags.contains(&tag)).collect();

    tags_get(Path(tag), State(state), talks, PageType::Talk).await
}
