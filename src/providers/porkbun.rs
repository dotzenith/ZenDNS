use super::DnsProvider;
use anyhow::{anyhow, Context, Result};
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PorkbunConfig {
    pub domain: String,
    pub subdomain: String,
    pub apikey: String,
    pub secretapikey: String,
    pub ttl: String
}

#[derive(Debug, Deserialize)]
struct PorkbunResponse {
    pub status: String
}

#[derive(Debug)]
pub struct PorkbunManager<'a> {
    client: &'a Client,
    config: PorkbunConfig,
}

impl<'a> PorkbunManager<'a> {
    pub fn new(client: &'a Client, config: PorkbunConfig) -> Self {
        PorkbunManager { client, config }
    }
}

impl<'a> DnsProvider for PorkbunManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let request = self
            .client
            .post(format!(
                "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/A/{}",
                self.config.domain, self.config.subdomain
            ))
            .json(&json!({
                "secretapikey": self.config.secretapikey,
                "apikey": self.config.apikey,
                "content": ip,
                "ttl": self.config.ttl
            }))
            .send()
            .context("Unable to send update request")?;

        let response: PorkbunResponse = request.json().context("Unable to parse API response")?;

        if response.status == "SUCCESS" {
            Ok(format!("Success! {}.{} set to {}", self.config.subdomain, self.config.domain, ip))

        } else {
            Err(anyhow!(format!("Update failed, response status: {}", response.status)))
        }
    }

    fn name(&self) -> &str {
        "porkbun"
    }
}
