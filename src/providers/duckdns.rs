use crate::common::get_ip;
use crate::config_manager::DuckDNSConfig;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct DuckdnsManager<'a> {
    client: &'a Client,
}

impl<'a> DuckdnsManager<'a> {
    pub fn new(client: &'a Client) -> Self {
        DuckdnsManager { client }
    }
    pub fn update_dns_record(&self, config: &DuckDNSConfig) -> Result<String> {
        let ip: String = get_ip()?;
        let response = self
            .client
            .get(format!(
                "https://www.duckdns.org/update?domains={}&token={}&ip={}",
                &config.domain, &config.token, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let text = response
            .text()
            .context("Could not convert response to text")?;

        if text == "OK" {
            Ok(format!("Success! {} has been set to {}", &config.domain, ip))
        } else {
            Err(anyhow!("Update Failed"))
        }
    }
}
