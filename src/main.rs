mod ip;
mod providers;
mod schema;
mod utils;

use clap::Parser;
use ip::get_ip;
use log::{error, info, warn};
use reqwest::blocking::Client;
use schema::Config;
use utils::{init_logger, read_ip, save_ip};

/// A CLI tool for managing Dynamic DNS updates
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// The json config file to use
    #[arg(short, long)]
    config: String,

    /// Where the output will be logged, uses stdout if not used
    #[arg(short, long)]
    log: Option<String>,

    /// Overrides the check for caching
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() {
    let args = Args::parse();

    let config = match Config::new(&args.config) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Config Error: {:?}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = init_logger(args.log.as_ref()) {
        eprintln!("Logger Error: {}", err);
        std::process::exit(1);
    }

    let ip = match get_ip() {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Unable to get IP: {:?}", err);
            std::process::exit(1);
        }
    };

    if !args.force {
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

    for provider in config.providers.into_iter() {
        let manager = provider.into_manager(&client);
        match manager.update(&ip.to_string()) {
            Ok(ok) => info!("{}: {}", manager.name(), ok),
            Err(err) => error!("{}: {}", manager.name(), err),
        }
    }
}
