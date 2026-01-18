use super::DnsProvider;
use anyhow::{Context, Result, anyhow};
use nutype::nutype;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[nutype(validate(predicate = validate_ttl), derive(Debug, Deserialize, Serialize))]
pub struct TTL(u32);

fn validate_ttl(value: &u32) -> bool {
    *value == 1 || (*value >= 60 && *value <= 86400)
}

fn default_ttl() -> TTL {
    TTL::try_new(1).unwrap()
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

#[derive(Debug)]
pub struct CloudflareManager<'a> {
    client: &'a Client,
    config: CloudflareConfig,
}

impl<'a> CloudflareManager<'a> {
    pub fn new(client: &'a Client, config: CloudflareConfig) -> Self {
        CloudflareManager { client, config }
    }
    fn response_successful(&self, json: &Value) -> Result<bool> {
        json["success"].as_bool().ok_or(anyhow!("No Success"))
    }

    pub fn get_zone_id(&self) -> Result<String> {
        const ZONES_ENDPOINT: &str = "https://api.cloudflare.com/client/v4/zones";
        let response = self
            .client
            .get(ZONES_ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.config.key))
            .send()
            .context("Could not get zones")?;

        let json: Value = response
            .json()
            .context("Could not parse response for zones")?;
        let success = self.response_successful(&json)?;

        if success {
            for zone in json["result"]
                .as_array()
                .context("Missing 'result' field in response despite success")?
            {
                if zone["name"]
                    .as_str()
                    .context("Missing 'name' field in Zone")?
                    == self.config.zone
                {
                    return Ok(zone["id"]
                        .as_str()
                        .context("Missing 'id' field in Zone")?
                        .to_string());
                }
            }
            return Err(anyhow!("Found no zones with matching name"));
        }
        let errors = json["errors"]
            .as_array()
            .context("No errors found despite failure")?;

        if errors.is_empty() {
            Err(anyhow!("An unspecified error occurred"))
        } else {
            Err(anyhow!(errors[0]["message"].to_string()))
        }
    }
    pub fn get_dns_record_id_and_ip(&self, zone_id: &str) -> Result<(String, String)> {
        let response = self
            .client
            .get(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A&name={}",
                zone_id, self.config.hostname
            ))
            .header("Authorization", format!("Bearer {}", self.config.key))
            .send()
            .context("Could not get DNS records")?;

        let json: Value = response
            .json()
            .context("Could not parse response for zones")?;
        let success = self.response_successful(&json)?;

        if success {
            let result_dict = &json["result"]
                .as_array()
                .context("Missing 'result' in response")?[0];
            let record_id = result_dict["id"]
                .as_str()
                .context("Missing 'id' for DNS record")?
                .to_string();
            let ip = result_dict["content"]
                .as_str()
                .context("Missing 'content' for DNS record")?
                .to_string();
            return Ok((record_id, ip));
        }

        let errors = json["errors"]
            .as_array()
            .context("No errors found despite failure")?;

        if errors.is_empty() {
            Err(anyhow!("An unspecified error occurred"))
        } else {
            Err(anyhow!(errors[0]["message"].to_string()))
        }
    }
}

impl<'a> DnsProvider for CloudflareManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let zone_id = self.get_zone_id()?;
        let (record_id, current_ip) = self.get_dns_record_id_and_ip(&zone_id)?;

        if current_ip == ip {
            return Ok(format!(
                "IP Address hasn't changed, no updates made to {}",
                self.config.hostname
            ));
        }

        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );
        let response = self
            .client
            .patch(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.key))
            .json(&json!({
                "content": ip,
                "name": self.config.hostname,
                "proxied": self.config.proxied,
                "type": "A",
                "ttl": self.config.ttl
            }))
            .send()?;

        let json: Value = response
            .json()
            .context("Could not parse response for zones")?;
        let success = self.response_successful(&json)?;

        if success {
            return Ok(format!(
                "Success! Hostname: {} for Zone: {} has been set to {}",
                self.config.hostname, self.config.zone, ip
            ));
        }
        Err(anyhow!("Update failed: {}", json.to_string()))
    }

    fn name(&self) -> &str {
        "cloudflare"
    }
}
