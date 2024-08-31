use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CloudflareConfig {
    pub key: String,
    pub zone: String,
    pub hostname: String,
    #[serde(deserialize_with = "validate_ttl")]
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

fn validate_ttl<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct TtlVisitor;

    impl<'de> serde::de::Visitor<'de> for TtlVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a u32 that is either 1, or between 60 and 86400")
        }

        fn visit_u64<E>(self, value: u64) -> Result<u32, E>
        where
            E: serde::de::Error,
        {
            if value == 1 || (value >= 60 && value <= 86400) {
                Ok(value as u32)
            } else {
                Err(E::custom(format!(
                    "TTL must be either 1, or between 60 and 86400, got {}",
                    value
                )))
            }
        }
    }

    deserializer.deserialize_u64(TtlVisitor)
}

pub fn config(path: &str) -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new(path, FileFormat::Yaml))
        .build()?;

    let settings: Settings = settings.try_deserialize()?;
    Ok(settings)
}
