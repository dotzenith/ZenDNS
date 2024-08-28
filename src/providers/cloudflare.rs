use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::env;
use crate::common::get_ip;

#[derive(Debug)]
pub struct CloudflareManager {
    api_key: String,
    zone_name: String,
    hostname: String,
    ttl: u32,
    proxied: bool,
    client: Client,
}

impl CloudflareManager {
    pub fn new() -> Result<Self> {
        let api_key: String = env::var("ZEN_API_KEY").context("API Key not found")?;
        let zone_name: String = env::var("ZEN_ZONE_NAME").context("Zone Name not found")?;
        let hostname: String = env::var("ZEN_HOSTNAME").context("Hostname not found")?;
        let ttl: u32 = env::var("ZEN_TTL")
            .context("API Key not found")?
            .parse()
            .context("TTL not a valid number")?;
        let proxied: bool = env::var("ZEN_PROXIED")
            .context("API Key not found")?
            .parse()
            .context("Proxied is not a boolean")?;
        let client = Client::new();

        Ok(CloudflareManager {
            api_key,
            zone_name,
            hostname,
            ttl,
            proxied,
            client,
        })
    }
    fn extract_record_id(&self, json: &Value) -> Result<bool> {
        Ok(json["success"].as_bool().ok_or(anyhow!("No Success"))? == true)
    }

    pub fn get_zone_id(&self) -> Result<String> {
        const ZONES_ENDPOINT: &str = "https://api.cloudflare.com/client/v4/zones";
        let response = self
            .client
            .get(ZONES_ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .context("Could not get zones")?;

        let json: Value = response.json().context("Could not parse response for zones")?;
        let success = self.extract_record_id(&json)?;

        if success {
            // Unwraps here are safe since the request status was success
            for zone in json["result"].as_array().unwrap() {
                if zone["name"].as_str().unwrap() == self.zone_name {
                    return Ok(zone["id"].as_str().unwrap().to_string());
                }
            }
            return Err(anyhow!("Found no zones with matching type"));
        }
        Err(anyhow!("Found no zones with matching type"))
    }
    pub fn get_dns_record_id_and_ip(&self) -> Result<(String, String)> {
        let zone_id = self.get_zone_id()?;
        let response = self
            .client
            .get(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A&name={}",
                zone_id, self.hostname
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .context("Could not get DNS records")?;

        let json: Value = response.json().context("Could not parse response for zones")?;
        let success = self.extract_record_id(&json)?;

        if success {
            // Unwraps here are safe since the request status was success
            let result_dict = &json["result"].as_array().unwrap()[0];
            let record_id = result_dict["id"]
                .as_str()
                .unwrap()
                .to_string();
            let ip = result_dict["content"]
                .as_str()
                .unwrap()
                .to_string();
            return Ok((record_id, ip))
        }
        Err(anyhow!("Could not find record id"))
    }
    pub fn update_dns_record(&self) -> Result<()> {
        let zone_id = self.get_zone_id()?;
        let (record_id, current_ip) = self.get_dns_record_id_and_ip()?;
        let ip = get_ip()?;

        if current_ip == ip {
            println!("Ip is the same, no need to update");
            return Ok(())
        }

        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );
        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "content": ip,
                "name": self.hostname,
                "proxied": self.proxied,
                "type": "A",
                "ttl": self.ttl
            }))
            .send()?;

        println!("Status: {}", response.status());
        Ok(())
    }
}
