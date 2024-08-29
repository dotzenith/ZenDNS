use anyhow::{Context, Result};
use reqwest::blocking::get;
use simplelog::{TermLogger, WriteLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use std::fs::File;
use std::process;

pub fn get_ip() -> Result<String> {
    Ok(get("https://icanhazip.com/")
        .context("Couldn't hit icanhazip")?
        .text()
        .context("Couldn't convert icanhazip output")?
        .trim()
        .to_owned())
}

pub fn init_logger(file: Option<&str>) {
    match file {
        Some(file_path) => {
            let file = File::create(file_path);
            if file.is_err() {
                eprintln!("Unable to create log file");
                process::exit(1);
            }
            let logger = WriteLogger::init(LevelFilter::Info, Config::default(), file.unwrap());
            if logger.is_err() {
                eprintln!("Unable to create log file");
                process::exit(1);
            }
            logger.unwrap();
        }
        None => {
            let logger = TermLogger::init(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Stdout,
                ColorChoice::Never,
            );
            if logger.is_err() {
                eprintln!("Unable to initialize logger for the terminal");
                process::exit(1);
            }
            logger.unwrap();
        }
    }
}
