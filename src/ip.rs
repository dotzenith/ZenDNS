use anyhow::{Context, Result};
use reqwest::blocking::get;
use serde_json::Value;

fn ipify() -> Result<String> {
    Ok(get("https://api.ipify.org")
        .context("Couldn't hit ipify")?
        .text()
        .context("Couldn't convert ipify output")?
        .trim()
        .to_owned())
}

fn icanhazip() -> Result<String> {
    Ok(get("https://icanhazip.com")
        .context("Couldn't hit icanhazip")?
        .text()
        .context("Couldn't convert icanhazip output")?
        .trim()
        .to_owned())
}

fn ifconfig() -> Result<String> {
    Ok(get("https://ifconfig.me/ip")
        .context("Couldn't hit ifconfig")?
        .text()
        .context("Couldn't convert ifconfig output")?
        .trim()
        .to_owned())
}

fn httpbin() -> Result<String> {
    let json: Value = get("https://httpbin.org/ip")
        .context("Couldn't hit httpbin")?
        .json()
        .context("Couldn't convert httpbin output")?;

    Ok(json["origin"]
        .as_str()
        .context("Failed to extract origin from JSON")?
        .to_string())
}

pub fn get_ip() -> Result<String> {
    ipify()
        .or_else(|_| icanhazip())
        .or_else(|_| ifconfig())
        .or_else(|_| httpbin())
}
