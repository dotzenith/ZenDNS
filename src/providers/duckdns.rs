use crate::common::get_ip;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct DuckdnsManager {
    client: Client,
}

impl DuckdnsManager {
    pub fn new() -> Result<Self> {
        let client = Client::new();

        Ok(DuckdnsManager { client })
    }
    pub fn update_dns_record(&self, token: String, domain: String) -> Result<()> {
        let ip: String = get_ip()?;
        let response = self
            .client
            .get(format!(
                "https://www.duckdns.org/update?domains={}&token={}&ip={}",
                domain, token, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let text = response.text().context("Could not convert response to text")?;

        if text == "OK" {
            Ok(())
        } else {
            Err(anyhow!("Update Failed"))
        }
    }
}
