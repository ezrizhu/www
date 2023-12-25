use reqwest;
use anyhow::Result;
use serde::Deserialize;
use rand::seq::SliceRandom;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub url: String,
}

pub async fn fetch() -> Result<Vec<Node>> {
    let response = reqwest::get("https://sitring.eric.si").await?;
    let ring: Vec<Node> = response.json().await?;
    Ok(ring)
}

pub fn get_neighbors(ring: Vec<Node>) -> (Node, Node) {
    let index = ring.iter().position(|x| x.id == "ezri").unwrap();
    let prev = ring.get((index + ring.len() - 1) % ring.len()).unwrap();
    let next = ring.get((index + 1) % ring.len()).unwrap();
    (prev.clone(), next.clone())
}

pub fn get_random(ring: Vec<Node>) -> Node {
    let (left, right) = get_neighbors(ring.clone());
    let ring: Vec<Node> = ring.clone().iter()
        .filter(|&node| node.id != "ezri")
        .filter(|&node| node.id != left.id)
        .filter(|&node| node.id != right.id)
        .cloned().collect();
    ring.choose(&mut rand::thread_rng()).unwrap().clone()
}
