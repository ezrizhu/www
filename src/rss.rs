use chrono::Utc;
use xml::writer::{EmitterConfig, XmlEvent};
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
    feeds.push(XmlEvent::start_element("rss").attr("version", "2.0").into());
    feeds.push(XmlEvent::start_element("channel").into());

    feeds.push(XmlEvent::start_element("title").into());
    feeds.push(XmlEvent::characters("Eric's blog").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("link").into());
    feeds.push(XmlEvent::characters("https://ericz.me/blog").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("description").into());
    feeds.push(XmlEvent::characters("Eric's thoughts on things.").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("language").into());
    feeds.push(XmlEvent::characters("en-us").into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("pubDate").into());
    let pubdate = Utc::now().to_rfc2822();
    feeds.push(XmlEvent::characters(&pubdate).into());
    feeds.push(XmlEvent::end_element().into());

    feeds.push(XmlEvent::start_element("generator").into());
    feeds.push(XmlEvent::characters("https://github.com/ericzty/www").into());
    feeds.push(XmlEvent::end_element().into());
    
    feeds.push(XmlEvent::start_element("ttl").into());
    feeds.push(XmlEvent::characters("1440").into());
    feeds.push(XmlEvent::end_element().into());

    for feed in feeds {
        writer.write(feed).unwrap();
    }

    for post in state.blog {
        feeds = Vec::new();
        let title = post.title;
        let link = format!("https://ericz.me/blog/{}", post.slug);
        let date = post.date.to_rfc2822();
        let content = post.body;

        feeds.push(XmlEvent::start_element("item").into());

        feeds.push(XmlEvent::start_element("title").into());
        feeds.push(XmlEvent::characters(&title).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("guid").into());
        feeds.push(XmlEvent::characters(&link).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("link").into());
        feeds.push(XmlEvent::characters(&link).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("description").into());
        feeds.push(XmlEvent::cdata(&content).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::start_element("pubDate").into());
        feeds.push(XmlEvent::characters(&date).into());
        feeds.push(XmlEvent::end_element().into());

        feeds.push(XmlEvent::end_element().into());
        for feed in feeds {
            writer.write(feed).unwrap();
        }
    }

    feeds = Vec::new();
    feeds.push(XmlEvent::end_element().into());
    feeds.push(XmlEvent::end_element().into());
    for feed in feeds {
        writer.write(feed).unwrap();
    }
    Response::builder()
        .header("Content-Type", "application/rss+xml")
        .body(body::boxed(body::Full::from(buf))).unwrap()
}
