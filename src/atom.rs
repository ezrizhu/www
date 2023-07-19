use xml::writer::{EmitterConfig, XmlEvent};
use chrono::Utc;
use axum::{
    body,
    extract::State,
    response::Response,
};

pub async fn get(State(state): State<super::SiteState>) -> Response {
    let mut buf: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buf);

    let mut feeds: Vec<XmlEvent> = Vec::new();
    feeds.push(XmlEvent::StartDocument{
        version: xml::common::XmlVersion::Version10,
        encoding: Some("UTF-8"),
        standalone: Some(true),
    });
    feeds.push(XmlEvent::start_element("feed").attr("xmlns", "http://www.w3.org/2005/Atom").into());

    feeds.push(XmlEvent::start_element("title").into());
    feeds.push(XmlEvent::characters("Eric's blog").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("id").into());
    feeds.push(XmlEvent::characters("https://ericz.me/blog.atom").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("updated").into());
    let updated = Utc::now().to_rfc3339();
    feeds.push(XmlEvent::characters(&updated).into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("author").into());
    feeds.push(XmlEvent::start_element("name").into());
    feeds.push(XmlEvent::characters("Tianyu (Eric) Zhu").into());
    feeds.push(XmlEvent::end_element().into());
    feeds.push(XmlEvent::start_element("email").into());
    feeds.push(XmlEvent::characters("eric@ericz.me").into());
    feeds.push(XmlEvent::end_element().into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("link")
               .attr("href", "https://ericz.me/blog.atom")
               .attr("rel", "self").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("link")
               .attr("href", "https://ericz.me/blog")
               .attr("rel", "alternate").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("generator").into());
    feeds.push(XmlEvent::characters("https://github.com/ericzty/www").into());
    feeds.push(XmlEvent::end_element().into());
    
    for feed in feeds {
        writer.write(feed).unwrap();
    }

    for post in state.blog {
        feeds = Vec::new();
        let title = post.title;
        let link = format!("https://ericz.me/blog/{}", post.slug);
        let date = post.date.to_rfc3339();
        let content = post.body;

        feeds.push(XmlEvent::start_element("item").into());

        feeds.push(XmlEvent::start_element("title").into());
        feeds.push(XmlEvent::characters(&title).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("id").into());
        feeds.push(XmlEvent::characters(&link).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("link").into());
        feeds.push(XmlEvent::characters(&link).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("published").into());
        feeds.push(XmlEvent::characters(&date).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("updated").into());
        feeds.push(XmlEvent::characters(&date).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("content")
                   .attr("type", "html")
                   .attr("xml:base", &link).into());
        feeds.push(XmlEvent::cdata(&content).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("link")
                   .attr("href", &link)
                   .attr("rel", "alternate").into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::end_element().into());
        for feed in feeds {
            writer.write(feed).unwrap();
        }
    }

    feeds = Vec::new();
    feeds.push(XmlEvent::end_element().into());
    for feed in feeds {
        writer.write(feed).unwrap();
    }
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(body::boxed(body::Full::from(buf))).unwrap()
}
