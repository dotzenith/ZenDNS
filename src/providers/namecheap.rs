use super::DnsProvider;
use crate::schema::NamecheapConfig;
use anyhow::{Context, Result, anyhow};
use regex::Regex;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct NamecheapManager<'a> {
    client: &'a Client,
    config: NamecheapConfig,
}

impl<'a> NamecheapManager<'a> {
    pub fn new(client: &'a Client, config: NamecheapConfig) -> Self {
        NamecheapManager { client, config }
    }
}

impl<'a> DnsProvider for NamecheapManager<'a> {
    fn update(&self, ip: &str) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "https://dynamicdns.park-your-domain.com/update?host={}&domain={}&password={}&ip={}",
                &self.config.host, &self.config.domain, &self.config.password, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let re = Regex::new(r"<IP>(\d+\.\d+\.\d+\.\d+)</IP>")?;
        let text = response
            .text()
            .context("Could not convert response to text")?;
        let captures = re
            .captures(&text)
            .context("Did not find any IP Addresses in response")?;

        if &captures[1] == ip {
            Ok(format!(
                "Success! Host: {} for Domain: {} has been set to {}",
                &self.config.host, &self.config.domain, ip
            ))
        } else {
            Err(anyhow!("IP Address returned by the XML does not match"))
        }
    }

    fn name(&self) -> &str {
        "namecheap"
    }
}
