use maud::{html, Markup};
use super::base;
use axum::extract::State;

pub async fn blog_index(State(state): State<super::SiteState>) -> Markup {
    let blog = state.blog.clone();
    let content = html! {
        h1 { "Blog" };
        p { "I have a " a href="/blog.xml" { "rss feed" } " and an " a href="/blog.atom" { "atom feed" } ", if you have a reader that supports them." }
        p { "You can also view the list of posts I have by their tags " a href="/blog/tags" { "here" } "." }
        @for blog in blog {
            div class="blog-box" {
                a href=(format!("/blog/{}", blog.slug)) { 
                    h2 { (blog.title) }
                    @let tags_str = blog.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                    p { "tags: " (tags_str) }

                    @let date_str = blog.date.format("%B %d, %Y").to_string();
                    p { (date_str) " - " (blog.description) }
                }
            }
        }
    };
    let extra_headers = html! {
        link rel="stylesheet" href="/assets/css/blog-index.css";
    };
    base("Blog", "My blog.", extra_headers, content, Some(state))
}
