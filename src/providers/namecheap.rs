use crate::common::get_ip;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct NamecheapManager {
    client: Client,
}

impl NamecheapManager {
    pub fn new() -> Result<Self> {
        let client = Client::new();

        Ok(NamecheapManager { client })
    }
    pub fn update_dns_record(&self, password: String, host: String, domain: String) -> Result<()> {
        let ip: String = get_ip()?;
        let response = self
            .client
            .get(format!(
                "https://dynamicdns.park-your-domain.com/update?host={}&domain={}&password={}&ip={}",
                host, domain, password, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let re = Regex::new(r"<IP>(\d+\.\d+\.\d+\.\d+)</IP>")?;
        let text = response
            .text()
            .context("Could not conver response to text")?;
        let captures = re
            .captures(&text)
            .context("Did not find any IP Addresses in response")?;

        if &captures[1] == ip {
            Ok(())
        } else {
            Err(anyhow!("IP Address returned by the XML does not match"))
        }
    }
}
