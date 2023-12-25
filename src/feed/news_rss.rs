use chrono::prelude::*;
use xml::writer::{EmitterConfig, XmlEvent};
use axum::{
    body,
    extract::State,
    response::Response,
};

pub async fn get(State(state): State<super::SiteState>) -> Response {
    let news_vec = state.news_vec.clone();

    let mut buf: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buf);

    let pubdate = Utc::now().to_rfc2822();
    let feeds: Vec<XmlEvent> = vec![
        XmlEvent::StartDocument{
            version: xml::common::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: Some(true),
        },
        XmlEvent::start_element("rss").attr("version", "2.0").into(),
        XmlEvent::start_element("channel").into(),

        XmlEvent::start_element("title").into(),
        XmlEvent::characters("Ezri's news").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("link").into(),
        XmlEvent::characters("https://ezrizhu.com/news").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("description").into(),
        XmlEvent::characters("Ezri's news.").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("language").into(),
        XmlEvent::characters("en-us").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("pubDate").into(),
        XmlEvent::characters(&pubdate).into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("generator").into(),
        XmlEvent::characters("https://github.com/ezrizhu/www").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("ttl").into(),
        XmlEvent::characters("1440").into(),
        XmlEvent::end_element().into(),
    ];

    for feed in feeds {
        writer.write(feed).unwrap();
    }

    let mut count = 0;
    let news_len = news_vec.len();
    for mut news in news_vec {
        // remove beginning bullet and space
        news.remove(0);
        news.remove(0);

        let (date_str, title) = news.split_once(": ").unwrap();
        let content = title.clone();
        // link (id) has to be unique
        let link = format!("{}#{}", "https://ezrizhu.com/news", (news_len - count).to_string());
        count+=1;

        let date = NaiveDate::parse_from_str(&format!("{} {}", date_str, "01"), "%b %Y %d").unwrap().and_hms_opt(0, 0, 0).unwrap();
        let date = DateTime::<Utc>::from_utc(date, Utc).to_rfc2822();

        let feeds: Vec<XmlEvent> = vec![
            XmlEvent::start_element("item").into(),

            XmlEvent::start_element("title").into(),
            XmlEvent::characters(&title).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("guid").into(),
            XmlEvent::characters(&link).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("link").into(),
            XmlEvent::characters(&link).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("description").into(),
            XmlEvent::cdata(&content).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("pubDate").into(),
            XmlEvent::characters(&date).into(),
            XmlEvent::end_element().into(),

            XmlEvent::end_element().into(),
        ];
        for feed in feeds {
            writer.write(feed).unwrap();
        }
    }

    let end: XmlEvent = XmlEvent::end_element().into();
    writer.write(end.clone()).unwrap();
    writer.write(end).unwrap();

    Response::builder()
        .header("Content-Type", "application/rss+xml")
        .body(body::boxed(body::Full::from(buf))).unwrap()
}
