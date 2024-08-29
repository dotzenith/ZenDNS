use crate::common::get_ip;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use std::env;

#[derive(Debug)]
pub struct NamecheapManager {
    password: String,
    host: String,
    domain: String,
    client: Client,
}

impl NamecheapManager {
    pub fn new() -> Result<Self> {
        let password: String = env::var("NAMECHEAP_PASSWORD").context("Password not found")?;
        let host: String = env::var("NAMECHEAP_HOST").context("Host not found")?;
        let domain: String = env::var("NAMECHEAP_DOMAIN").context("Domain not found")?;
        let client = Client::new();

        Ok(NamecheapManager {
            password,
            host,
            domain,
            client,
        })
    }
    pub fn update_dns_record(&self) -> Result<()> {
        let ip: String = get_ip()?;
        let response = self
            .client
            .get(format!(
                "https://dynamicdns.park-your-domain.com/update?host={}&domain={}&password={}&ip={}",
                self.host, self.domain, self.password, ip
            ))
            .send()
            .context("Could not get DNS records")?;

        let re = Regex::new(r"<IP>(\d+\.\d+\.\d+\.\d+)</IP>")?;
        let text = response.text().context("Could not conver response to text")?;
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
