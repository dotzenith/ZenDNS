use anyhow::{Context, Result};
use bincode::{deserialize_from, serialize_into};
use platform_dirs::AppDirs;
use reqwest::blocking::get;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::{create_dir, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::process;

pub fn get_ip() -> Result<String> {
    Ok(get("https://icanhazip.com/")
        .context("Couldn't hit icanhazip")?
        .text()
        .context("Couldn't convert icanhazip output")?
        .trim()
        .to_owned())
}

pub fn init_logger(file: Option<&String>) {
    match file {
        Some(file_path) => {
            let file = OpenOptions::new().create(true).append(true).open(file_path);

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

pub fn save_ip(ip: &str) -> Result<()> {
    let app_dirs = AppDirs::new(Some("ZenDNS"), true).context("Unable to get cache directory")?;
    if !app_dirs.cache_dir.exists() {
        create_dir(&app_dirs.cache_dir).context("Unable to create cache directory")?;
    }

    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(app_dirs.cache_dir.join("ip")).context("Unable to create cache file")?,
    );

    serialize_into(&mut file, ip).context("Unable to save IP to file")?;
    Ok(())
}

pub fn read_ip() -> Result<String> {
    let app_dirs = AppDirs::new(Some("ZenDNS"), true).context("Unable to get cache directory")?;
    let mut file = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(app_dirs.cache_dir.join("ip")).context("IP cache does not exist")?,
    );

    let stations: String = deserialize_from(&mut file).context("Unable to read IP from file")?;
    Ok(stations)
}
