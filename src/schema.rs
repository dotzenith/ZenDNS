use anyhow::{Context, Result};
use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::fs;

#[nutype(validate(predicate = validate_ttl), derive(Debug, Deserialize, Serialize))]
pub struct TTL(u32);

fn validate_ttl(value: &u32) -> bool {
    *value == 1 || (*value >= 60 && *value <= 86400)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub providers: Vec<Provider>,
}

impl Config {
    pub fn new(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).context("Unable to read config file")?;
        serde_json::from_str(&content).context("Invalid config file")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Provider {
    Cloudflare(CloudflareConfig),
    Namecheap(NamecheapConfig),
    DuckDNS(DuckDNSConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareConfig {
    pub key: String,
    pub zone: String,
    pub hostname: String,
    #[serde(default = "default_ttl")]
    pub ttl: TTL,
    #[serde(default)]
    pub proxied: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamecheapConfig {
    pub password: String,
    pub host: String,
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuckDNSConfig {
    pub token: String,
    pub domain: String,
}

fn default_ttl() -> TTL {
    TTL::try_new(1).unwrap()
}
