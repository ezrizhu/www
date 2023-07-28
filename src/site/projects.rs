use maud::{html, Markup};
use super::base;
use axum::extract::State;

pub async fn project_index(State(state): State<super::SiteState>) -> Markup {
    let projects = state.projects.clone();
    let content = html! {
        div class="box" {
            h1 { "Projects" };
            p class="desc" { "You can also view the list of projects I have by their tags " a href="/projects/tags" { "here" } "." }
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
    base("Projects", "A list of projects I've worked on", extra_headers, content, Some(state))
}
