mod cloudflare;
mod duckdns;
mod namecheap;
mod porkbun;
use anyhow::Result;
pub use cloudflare::{CloudflareConfig, CloudflareManager};
pub use duckdns::{DuckDNSConfig, DuckdnsManager};
pub use namecheap::{NamecheapConfig, NamecheapManager};
pub use porkbun::{PorkbunConfig, PorkbunManager};

pub trait DnsProvider {
    fn update(&self, ip: &str) -> Result<String>;
    fn name(&self) -> &str;
}
