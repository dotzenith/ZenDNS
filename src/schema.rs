use crate::providers::*;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;

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
    Porkbun(PorkbunConfig),
}

impl Provider {
    pub fn into_manager(self, client: &Client) -> Box<dyn DnsProvider + '_> {
        match self {
            Provider::Cloudflare(conf) => Box::new(CloudflareManager::new(client, conf)),
            Provider::Namecheap(conf) => Box::new(NamecheapManager::new(client, conf)),
            Provider::DuckDNS(conf) => Box::new(DuckdnsManager::new(client, conf)),
            Provider::Porkbun(conf) => Box::new(PorkbunManager::new(client, conf)),
        }
    }
}
