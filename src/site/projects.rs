use maud::{html, Markup};
use super::base;
use axum::extract::State;

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
