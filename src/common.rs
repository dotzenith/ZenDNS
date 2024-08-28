use anyhow::{Context, Result};
use reqwest::blocking::get;

pub fn get_ip() -> Result<String> {
    Ok(get("https://icanhazip.com/")
        .context("Couldn't hit icanhazip")?
        .text()
        .context("Couldn't convert icanhazip output")?
        .trim()
        .to_owned())
}
