use super::DnsProvider;
use crate::schema::DuckDNSConfig;
use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct DuckdnsManager<'a> {
    name: &'static str,
    client: &'a Client,
    config: DuckDNSConfig,
}

impl<'a> DuckdnsManager<'a> {
    pub fn new(client: &'a Client, config: DuckDNSConfig) -> Self {
        DuckdnsManager {
            name: "duckdns",
            client,
            config,
        }
    }
}

impl<'a> DnsProvider for DuckdnsManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "https://www.duckdns.org/update?domains={}&token={}&ip={}",
                &self.config.domain, &self.config.token, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let text = response
            .text()
            .context("Could not convert response to text")?;

        if text == "OK" {
            Ok(format!(
                "Success! {} has been set to {}",
                &self.config.domain, ip
            ))
        } else {
            Err(anyhow!("Update Failed"))
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}
