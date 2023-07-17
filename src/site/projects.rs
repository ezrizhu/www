use maud::{html, Markup, PreEscaped};
use super::not_found::not_found;
use crate::projects::get_all;
use super::base;
use axum::{
    extract::{Path,State},
    http::StatusCode
};

pub async fn project_handler(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {
    if let Some(project) = get_all(state.projects, &name) {
        (StatusCode::OK, project_page(&project.title, &project.description, &project.body))
    } else {
        (StatusCode::NOT_FOUND, not_found().await)
    }
}

fn project_page(title: &str, desc: &str, body: &str) -> Markup {
    let content = html! {
        h1 { (title) };
        p { (PreEscaped(body)) };
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post.css";
    };
    base(title, desc, extra_headers, content)
}

pub async fn project_index(State(state): State<super::SiteState>) -> Markup {
    let projects = state.projects;
    let content = html! {
        h1 { "Projects" };
        div class="pure-g" {
            @for project in projects {
                div class="pure-u-1 pure-u-md-1-3" {
                    div class="project-box" {
                        a href=(format!("/projects/{}", project.slug)) { 
                            h2 { (project.title) }
                            p { (project.description) }
                        }
                    }
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/projects-index.css";
    };
    base("Projects", "A list of projects I've worked on", extra_headers, content)
}
