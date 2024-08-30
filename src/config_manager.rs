use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudflareConfig {
    pub key: String,
    pub zone: String,
    pub hostname: String,
    pub ttl: u32,
    pub proxied: bool,
}

#[derive(Debug, Deserialize)]
pub struct NamecheapConfig {
    pub password: String,
    pub host: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct DuckDNSConfig {
    pub token: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub cloudflare: Option<Vec<CloudflareConfig>>,
    pub namecheap: Option<Vec<NamecheapConfig>>,
    pub duckdns: Option<Vec<DuckDNSConfig>>,
}

pub fn config(path: &str) -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new(path, FileFormat::Yaml))
        .build()?;

    let settings: Settings = settings.try_deserialize()?;
    Ok(settings)
}
