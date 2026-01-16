use anyhow::{Context, Result};
use reqwest::blocking::get;
use serde_json::Value;
use std::net::Ipv4Addr;

fn ipify() -> Result<Ipv4Addr> {
    let response = get("https://api.ipify.org")
        .context("Couldn't hit ipify")?
        .text()
        .context("Couldn't convert ipify output")?
        .trim()
        .to_owned();
    Ok(response.parse::<Ipv4Addr>()?)
}

fn icanhazip() -> Result<Ipv4Addr> {
    let response = get("https://icanhazip.com")
        .context("Couldn't hit icanhazip")?
        .text()
        .context("Couldn't convert icanhazip output")?
        .trim()
        .to_owned();
    Ok(response.parse::<Ipv4Addr>()?)
}

fn ifconfig() -> Result<Ipv4Addr> {
    let response = get("https://ifconfig.me/ip")
        .context("Couldn't hit ifconfig")?
        .text()
        .context("Couldn't convert ifconfig output")?
        .trim()
        .to_owned();
    Ok(response.parse::<Ipv4Addr>()?)
}

fn httpbin() -> Result<Ipv4Addr> {
    let json: Value = get("https://httpbin.org/ip")
        .context("Couldn't hit httpbin")?
        .json()
        .context("Couldn't convert httpbin output")?;

    let ip = json["origin"]
        .as_str()
        .context("Failed to extract origin from JSON")?
        .to_string();
    Ok(ip.parse::<Ipv4Addr>()?)
}

pub fn get_ip() -> Result<Ipv4Addr> {
    ipify()
        .or_else(|_| icanhazip())
        .or_else(|_| ifconfig())
        .or_else(|_| httpbin())
}
