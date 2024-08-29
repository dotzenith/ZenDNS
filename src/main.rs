mod common;
mod config_manager;
mod providers;

use crate::providers::{CloudflareManager, DuckdnsManager, NamecheapManager};

fn main() {
    println!("{}", common::get_ip().unwrap());
    let settings = config_manager::config().unwrap();

    if let Some(cloudflare) = settings.cloudflare {
        for entry in cloudflare.into_iter() {
            CloudflareManager::new()
                .unwrap()
                .update_dns_record(
                    entry.api_key,
                    entry.zone_name,
                    entry.hostname,
                    entry.ttl,
                    entry.proxied,
                )
                .unwrap()
        }
    }

    if let Some(namecheap) = settings.namecheap {
        for entry in namecheap.into_iter() {
            NamecheapManager::new()
                .unwrap()
                .update_dns_record(entry.password, entry.host, entry.domain)
                .unwrap()
        }
    }

    if let Some(duckdns) = settings.duckdns {
        for entry in duckdns.into_iter() {
            DuckdnsManager::new()
                .unwrap()
                .update_dns_record(entry.token, entry.domain)
                .unwrap()
        }
    }
}
