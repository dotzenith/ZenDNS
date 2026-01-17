mod cloudflare;
mod duckdns;
mod namecheap;
use anyhow::Result;
pub use cloudflare::CloudflareManager;
pub use duckdns::DuckdnsManager;
pub use namecheap::NamecheapManager;

pub trait DnsProvider {
    fn update(&self, ip: &str) -> Result<String>;
    fn name(&self) -> &str;
}
