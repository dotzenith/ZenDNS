use crate::config_manager::NamecheapConfig;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct NamecheapManager<'a> {
    client: &'a Client,
}

impl<'a> NamecheapManager<'a> {
    pub fn new(client: &'a Client) -> Self {
        NamecheapManager { client }
    }
    pub fn update(&self, config: &NamecheapConfig, ip: &str) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "https://dynamicdns.park-your-domain.com/update?host={}&domain={}&password={}&ip={}",
                &config.host, &config.domain, &config.password, ip
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
            Ok(format!("Success! Host: {} for Domain: {} has been set to {}", &config.host, &config.domain, ip))
        } else {
            Err(anyhow!("IP Address returned by the XML does not match"))
        }
    }
}
