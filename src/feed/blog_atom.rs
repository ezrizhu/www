use xml::writer::{EmitterConfig,XmlEvent};
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

    let updated = Utc::now().to_rfc3339();

    let feeds: Vec<XmlEvent> = vec![
        XmlEvent::StartDocument{
            version: xml::common::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: Some(true),
        },
        XmlEvent::start_element("feed").attr("xmlns", "http://www.w3.org/2005/Atom").into(),

        XmlEvent::start_element("title").into(),
        XmlEvent::characters("ezri's blog").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("id").into(),
        XmlEvent::characters("https://ezrizhu.com/blog.atom").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("updated").into(),
        XmlEvent::characters(&updated).into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("author").into(),
        XmlEvent::start_element("name").into(),
        XmlEvent::characters("ezri zhu").into(),
        XmlEvent::end_element().into(),
        XmlEvent::start_element("email").into(),
        XmlEvent::characters("me@ezrizhu.com").into(),
        XmlEvent::end_element().into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("link")
            .attr("href", "https://ezrizhu.com/blog.atom")
            .attr("rel", "self").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("link")
            .attr("href", "https://ezrizhu.com/blog")
            .attr("rel", "alternate").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("generator").into(),
        XmlEvent::characters("https://github.com/ezrizhu/www").into(),
        XmlEvent::end_element().into(),
        ];

    for feed in feeds {
        writer.write(feed).unwrap();
    }

    for post in state.blog {
        let title = post.title;
        let link = format!("https://ezrizhu.com/blog/{}", post.slug);
        let date = post.date.to_rfc3339();
        let content = post.body;

        let feeds: Vec<XmlEvent> = vec![
            XmlEvent::start_element("entry").into(),

            XmlEvent::start_element("title").into(),
            XmlEvent::characters(&title).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("id").into(),
            XmlEvent::characters(&link).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("published").into(),
            XmlEvent::characters(&date).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("updated").into(),
            XmlEvent::characters(&date).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("content")
                .attr("type", "html")
                .attr("xml:base", &link).into(),
            XmlEvent::cdata(&content).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("link")
                .attr("href", &link)
                .attr("rel", "alternate").into(),
            XmlEvent::end_element().into(),

            XmlEvent::end_element().into(),
            ];

        for feed in feeds {
            writer.write(feed).unwrap();
        }
    }

    let end: XmlEvent = XmlEvent::end_element().into();
    writer.write(end).unwrap();

    Response::builder()
        .header("Content-Type", "application/xml")
        .body(body::boxed(body::Full::from(buf))).unwrap()
}

