use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudflareConfig {
    pub api_key: String,
    pub zone_name: String,
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

pub fn config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("example.yaml", FileFormat::Yaml))
        .build()?;

    let settings: Settings = settings.try_deserialize()?;
    Ok(settings)
}
