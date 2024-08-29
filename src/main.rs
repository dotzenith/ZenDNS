mod common;
mod config_manager;
mod providers;

use crate::providers::{CloudflareManager, DuckdnsManager, NamecheapManager};
use reqwest::blocking::Client;
use common::init_logger;
use log::{error, info};

fn main() {
    let settings = config_manager::config().unwrap();
    let client = Client::new();
    init_logger(None);

    if let Some(cloudflare) = settings.cloudflare {
        for config in cloudflare.iter() {
            match CloudflareManager::new(&client).update_dns_record(config) {
                Ok(ok) => info!("Cloudflare: {}", ok),
                Err(err) => error!("Cloudflare: {}", err)
            }
        }
    }

    if let Some(namecheap) = settings.namecheap {
        for config in namecheap.iter() {
            match NamecheapManager::new(&client).update_dns_record(config) {
                Ok(ok) => info!("Namecheap: {}", ok),
                Err(err) => error!("Namecheap: {}", err)
            }
        }
    }

    if let Some(duckdns) = settings.duckdns {
        for config in duckdns.iter() {
            match DuckdnsManager::new(&client).update_dns_record(config) {
                Ok(ok) => info!("Duckdns: {}", ok),
                Err(err) => error!("Duckdns: {}", err)
            }
        }
    }
}
