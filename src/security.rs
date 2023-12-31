use std::fs;
use axum::{
    http::HeaderMap,
};

pub async fn securitytxt() -> (HeaderMap, String) {
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "text/plain".parse().unwrap());
    resp_header.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    let securitytxt = fs::read_to_string("assets/security.txt").unwrap();
    (resp_header, securitytxt)
}
