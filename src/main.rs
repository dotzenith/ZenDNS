mod config_manager;
mod ip;
mod providers;
mod utils;

use crate::providers::{CloudflareManager, DuckdnsManager, NamecheapManager};
use clap::{arg, command, ArgAction};
use ip::get_ip;
use log::{error, info, warn};
use reqwest::blocking::Client;
use utils::{init_logger, read_ip, save_ip};

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
        .arg(
            arg!(-f --force ... "Overrides the check for caching")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let config: &String = matches.get_one("config").unwrap();
    let settings = match config_manager::config(config) {
        Ok(set) => set,
        Err(err) => {
            eprintln!("Config Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let log: Option<&String> = matches.get_one("log");
    init_logger(log); // Will exit if it doesn't succeed

    let ip = match get_ip() {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Unable to get IP: {:?}", err);
            std::process::exit(1);
        }
    };

    if !matches.get_flag("force") {
        match read_ip() {
            Ok(saved_ip) => {
                if saved_ip == ip {
                    info!("IP is the same as last usage, no updates will be made");
                    std::process::exit(0);
                }
            }
            Err(err) => {
                warn!("{}: records will still be updated", err);
            }
        }
    }

    match save_ip(&ip) {
        Ok(_) => (),
        Err(err) => {
            warn!("{}: records will still be updated", err);
        }
    }

    let client = Client::new();
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
