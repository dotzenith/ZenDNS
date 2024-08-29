mod common;
mod config_manager;
mod providers;

use crate::providers::{CloudflareManager, DuckdnsManager, NamecheapManager};
use reqwest::blocking::Client;

fn main() {
    println!("{}", common::get_ip().unwrap());
    let settings = config_manager::config().unwrap();
    let client = Client::new();

    if let Some(cloudflare) = settings.cloudflare {
        for config in cloudflare.iter() {
            CloudflareManager::new(&client)
                .unwrap()
                .update_dns_record(config)
                .unwrap()
        }
    }

    if let Some(namecheap) = settings.namecheap {
        for config in namecheap.iter() {
            NamecheapManager::new(&client)
                .unwrap()
                .update_dns_record(&config)
                .unwrap()
        }
    }

    if let Some(duckdns) = settings.duckdns {
        for config in duckdns.iter() {
            DuckdnsManager::new(&client)
                .unwrap()
                .update_dns_record(config)
                .unwrap()
        }
    }
}
