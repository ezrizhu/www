use maud::{html, Markup};
use super::base;
use axum::extract::State;

pub async fn blog_index(State(state): State<super::SiteState>) -> Markup {
    let blog = state.blog.clone();
    let content = html! {
        div class="box" {
            h1 { "Blog" };
            p class="desc" {
                "I have a " a href="/blog.xml" { "rss feed" } " and an " a href="/blog.atom" { "atom feed" } ", if you have a reader that supports them."
                br;
                "You can also view the list of posts I have by their tags " a href="/blog/tags" { "here" } "."
            }
            ul {
                @for blog in blog {
                    li {
                        p {
                            @let date_str = blog.date.format("%B %d, %Y").to_string();
                            (date_str) ": "
                                a href=(format!("/blog/{}", blog.slug)) { 
                                    { (blog.title) }
                                }
                            " - "
                            (blog.description)
                        }
                    }
                }
            };
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/post-index.css";
    };
    base("Blog", "My blog.", extra_headers, content, Some(state))
}
