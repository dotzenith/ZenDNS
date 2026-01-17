mod ip;
mod providers;
mod schema;
mod utils;

use clap::{ArgAction, arg, command};
use ip::get_ip;
use log::{error, info, warn};
use reqwest::blocking::Client;
use schema::Config;
use utils::{init_logger, read_ip, save_ip};

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .arg(
            arg!(-c --config <CONFIG>)
                .required(true)
                .help("The json config file to use"),
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

    let config_path: &String = matches.get_one("config").unwrap();
    let config = match Config::new(config_path) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Config Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let log: Option<&String> = matches.get_one("log");
    if let Err(err) = init_logger(log) {
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

    for provider in config.providers.into_iter() {
        let manager = provider.into_manager(&client);
        match manager.update(&ip.to_string()) {
            Ok(ok) => info!("{}: {}", manager.name(), ok),
            Err(err) => error!("{}: {}", manager.name(), err),
        }
    }
}
