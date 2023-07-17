use maud::{html, Markup};
use super::not_found::not_found;
use crate::post::get_all;
use super::base;
use super::post::post;
use axum::{
    extract::{Path,State},
    http::StatusCode
};

pub async fn project_handler(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, Markup) {
    if let Some(project) = get_all(state.projects, &name) {
        (StatusCode::OK, post(&project.title, &project.date, &project.description, &project.body))
    } else {
        (StatusCode::NOT_FOUND, not_found().await)
    }
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
                            h3 { (project.date) }
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
