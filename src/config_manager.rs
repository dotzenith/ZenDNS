use config::{Config, ConfigError, File, FileFormat};
use nutype::nutype;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[nutype(validate(predicate = validate_ttl), derive(Debug, Deserialize, Serialize))]
struct TTL(u32);

#[derive(Debug, Deserialize)]
pub struct CloudflareConfig {
    pub key: String,
    pub zone: String,
    pub hostname: String,
    pub ttl: TTL,
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

fn validate_ttl(value: &u32) -> bool {
    *value == 1 || (*value >= 60 && *value <= 86400)
}
