use std::fs;
use axum::{
    http::HeaderMap,
};

pub async fn sshpub() -> (HeaderMap, String) {
    let mut resp_header = HeaderMap::new();
    resp_header.insert("Content-Type", "text/plain".parse().unwrap());
    resp_header.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    let sshraw = fs::read_to_string("assets/ssh").unwrap();
    (resp_header, sshraw)
}
