use super::DnsProvider;
use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PorkbunConfig {
    pub domain: String,
    pub subdomain: String,
    pub apikey: String,
    pub secretapikey: String,
    pub ttl: String,
}

#[derive(Debug)]
pub struct PorkbunManager<'a> {
    client: &'a Client,
    config: PorkbunConfig,
}

#[derive(Debug, Deserialize)]
struct EditResponse {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RetrieveResponse {
    pub cloudflare: String,
    pub records: Vec<RetrieveRecord>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RetrieveRecord {
    pub content: String,
    pub id: String,
    pub name: String,
}

impl<'a> PorkbunManager<'a> {
    pub fn new(client: &'a Client, config: PorkbunConfig) -> Self {
        PorkbunManager { client, config }
    }
    pub fn current_ip(&self) -> Result<String> {
        let request = self
            .client
            .post(format!(
                "https://api.porkbun.com/api/json/v3/dns/retrieveByNameType/{}/A/{}",
                self.config.domain, self.config.subdomain
            ))
            .json(&json!({
                "secretapikey": self.config.secretapikey,
                "apikey": self.config.apikey,
            }))
            .send()
            .context("Unable to send fetch request")?;

        let response: RetrieveResponse = request.json().context("Unable to parse API response")?;

        if response.status != "SUCCESS" {
            Err(anyhow!(format!(
                "Fetch failed, response status: {}",
                response.status
            )))
        } else {
            Ok(response.records[0].content.clone())
        }
    }
}

impl<'a> DnsProvider for PorkbunManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let current_ip = self.current_ip()?;

        if current_ip == ip {
            return Ok(format!(
                "{}.{} already set to {}, no changes made",
                self.config.subdomain, self.config.domain, ip
            ));
        }

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

        let response: EditResponse = request.json().context("Unable to parse API response")?;

        if response.status == "SUCCESS" {
            Ok(format!(
                "Success! {}.{} set to {}",
                self.config.subdomain, self.config.domain, ip
            ))
        } else {
            Err(anyhow!(format!(
                "Update failed, response status: {}",
                response.status
            )))
        }
    }

    fn name(&self) -> &str {
        "porkbun"
    }
}
