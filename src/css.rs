use std::fs;
use minifier::css::minify;
use axum::{
    extract::{Path,State},
    http::{StatusCode,header::HeaderMap}
};

#[derive(Clone)]
pub struct Css {
    name: String,
    css: String,
}

fn css_minify(input: String) -> String {
    minify(&input).expect("minify failure").to_string()
}

pub fn init() -> Vec<Css> {
    let mut css_files = vec![];

    if let Ok(entries) = fs::read_dir("css/") {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let name = name.replace("_", "-");
                let name = name.replace(".css", "");
                let css = fs::read_to_string(path).unwrap();
                let css = css_minify(css);
                let css = Css { name, css };
                css_files.push(css);
            }
        }
    }
    css_files
}


fn get_css_by_name<'a>(css_files: &'a Vec<Css>, name: &'a str) -> Option<&'a str> {
    for css in css_files {
        if css.name == name {
            return Some(&css.css);
        }
    }
    None
}

pub fn make_css(want_list: Vec<&str>, css_files: &Vec<Css>) -> String {
    let mut css = String::new();
    for name in want_list {
        if let Some(css_str) = get_css_by_name(css_files, name) {
            css.push_str(css_str);
            css.push('\n');
        }
    }
    css
}

pub async fn get(Path(name): Path<String>, State(state): State<super::SiteState>) -> (StatusCode, HeaderMap, String) {
    let mut resp_status = StatusCode::OK;
    let mut resp_body = String::new();
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "text/css".parse().unwrap());
    resp_header.insert("Cache-Control", "max-age=86400".parse().unwrap());
    match name.replace(".css", "").as_str() {
        "main" => resp_body = make_css(vec!["nord", "pure-min", "main"], &state.css),
        "home" => resp_body = make_css(vec!["grids-responsive-min", "home"], &state.css),
        // contact is currently not being used
        "contact" => resp_body = make_css(vec!["grids-responsive-min", "contact"], &state.css),
        "news" => resp_body = make_css(vec!["news"], &state.css),
        "post-index" => resp_body = make_css(vec!["post-index"], &state.css),
        "post" => resp_body = make_css(vec!["post"], &state.css),
        "404" => resp_body = make_css(vec!["404"], &state.css),
        "wip" => resp_body = make_css(vec!["wip"], &state.css),
        _ => resp_status = StatusCode::NOT_FOUND
    };

    (resp_status, resp_header, resp_body)
}
