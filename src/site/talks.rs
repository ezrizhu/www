use maud::{html, Markup};
use super::base;
use axum::extract::State;

pub async fn talk_index(State(state): State<super::SiteState>) -> Markup {
    let talks = state.talks.clone();
    let content = html! {
        div class="box" {
            h1 { "Talks" };
            p class="desc" { "You can also view the list of talks I have by their tags " a href="/talks/tags" { "here" } "." }
            ul {
                @for talk in talks {
                    li {
                        p {
                            a href=(format!("/talks/{}", talk.slug)) { 
                                { (talk.title) }
                            }
                            " - "
                            (talk.description)
                        }
                    }
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
        link rel="canonical" href="https://ezrizhu.com/talks";
    };
    base("Talks", "A list of talks I have given.", extra_headers, content, Some(state))
}
