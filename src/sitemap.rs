use sitemap;
use axum::{
    body,
    extract::State,
    response::Response,
};
use anyhow::Result;

pub fn init(state: super::SiteState) -> Result<Vec<u8>> {
    let mut sm: Vec<u8> = Vec::new();
    let smw = sitemap::writer::SiteMapWriter::new(&mut sm);
    let mut urlwriter = smw.start_urlset()?;
    urlwriter.url("https://ezrizhu.com")?;
    let static_pages = vec!["contact", "news", "projects", "blog", "now", "projects/tags", "blog/tags", "talks", "talks/tags"];
    for page in static_pages {
        urlwriter.url(format!("https://ezrizhu.com/{}", page))?;
    }
    for project in state.projects {
        urlwriter.url(format!("https://ezrizhu.com/projects/{}", project.slug))?;
    }
    for blog in state.blog {
        urlwriter.url(format!("https://ezrizhu.com/blog/{}", blog.slug))?;
    }
    for talk in state.talks {
        urlwriter.url(format!("https://ezrizhu.com/talks/{}", talk.slug))?;
    }
    urlwriter.end()?;
    Ok(sm)
}

pub async fn get(State(state): State<super::SiteState>) -> Response {
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(body::boxed(body::Full::from(state.sitemap.clone()))).unwrap()
}
