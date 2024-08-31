mod common;
mod config_manager;
mod providers;

use crate::providers::{CloudflareManager, DuckdnsManager, NamecheapManager};
use clap::{arg, command};
use common::{init_logger, get_ip};
use log::{error, info};
use reqwest::blocking::Client;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .arg(
            arg!(-c --config <CONFIG>)
                .required(true)
                .help("The yaml config file to use"),
        )
        .arg(
            arg!(-l --log <LOGFILE>)
                .required(false)
                .help("Where the output will be logged, uses stdout if not used"),
        )
        .get_matches();

    // Required so unwrap is swell
    let config: &String = matches.get_one("config").unwrap();
    let log: Option<&String> = matches.get_one("log");

    let settings = match config_manager::config(config) {
        Ok(set) => set,
        Err(err) => {
            eprintln!("Config Error: {:?}", err);
            std::process::exit(1);
        }
    };
    let client = Client::new();
    init_logger(log); // Will exit if it doesn't succeed

    let ip = match get_ip() {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Unable to get IP: {:?}", err);
            std::process::exit(1);
        }
    };

    if let Some(cloudflare) = settings.cloudflare {
        for config in cloudflare.iter() {
            match CloudflareManager::new(&client).update(config, &ip) {
                Ok(ok) => info!("Cloudflare: {}", ok),
                Err(err) => error!("Cloudflare: {}", err),
            }
        }
    }

    if let Some(namecheap) = settings.namecheap {
        for config in namecheap.iter() {
            match NamecheapManager::new(&client).update(config, &ip) {
                Ok(ok) => info!("Namecheap: {}", ok),
                Err(err) => error!("Namecheap: {}", err),
            }
        }
    }

    if let Some(duckdns) = settings.duckdns {
        for config in duckdns.iter() {
            match DuckdnsManager::new(&client).update(config, &ip) {
                Ok(ok) => info!("Duckdns: {}", ok),
                Err(err) => error!("Duckdns: {}", err),
            }
        }
    }
}
