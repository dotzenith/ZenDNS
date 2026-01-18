use super::DnsProvider;
use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DuckDNSConfig {
    pub token: String,
    pub domain: String,
}

#[derive(Debug)]
pub struct DuckdnsManager<'a> {
    client: &'a Client,
    config: DuckDNSConfig,
}

impl<'a> DuckdnsManager<'a> {
    pub fn new(client: &'a Client, config: DuckDNSConfig) -> Self {
        DuckdnsManager { client, config }
    }
}

impl<'a> DnsProvider for DuckdnsManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "https://www.duckdns.org/update?domains={}&token={}&ip={}",
                self.config.domain, self.config.token, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let text = response
            .text()
            .context("Could not convert response to text")?;

        if text == "OK" {
            Ok(format!(
                "Success! {} has been set to {}",
                self.config.domain, ip
            ))
        } else {
            Err(anyhow!("Update Failed"))
        }
    }

    fn name(&self) -> &str {
        "duckdns"
    }
}
