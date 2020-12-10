//! Types and common functions to use API from hub.docker.com

use chrono::prelude::*;
use std::time::Duration;

pub fn http_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent(concat!("dhrb/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
}

#[derive(serde::Deserialize, Debug)]
pub struct Summary {
    pub name: String,
    pub slug: String,
    pub updated_at: DateTime<Utc>,
    pub pull_count: Option<String>,
    pub star_count: Option<usize>,
    pub short_description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Repository {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub star_count: Option<usize>,
    pub pull_count: Option<usize>,
    pub full_description: Option<String>,
    pub is_automated: Option<bool>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub images: Vec<Image>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Image {
    pub architecture: String,
    pub os: String,
    pub size: u64,
    pub digest: Option<String>,
}
