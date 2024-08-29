mod cloudflare;
mod namecheap;
mod duckdns;
pub use cloudflare::CloudflareManager;
pub use namecheap::NamecheapManager;
pub use duckdns::DuckdnsManager;
