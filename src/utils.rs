use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::{OpenOptions, create_dir_all, read, write};
use std::net::Ipv4Addr;
use std::path::PathBuf;

fn cache_dir() -> Result<PathBuf> {
    ProjectDirs::from("com", "dotzenith", "ZenDNS")
        .map(|dirs| dirs.cache_dir().to_path_buf())
        .ok_or_else(|| anyhow!("Unable to get App Directory"))
}

pub fn init_logger(file: Option<&String>) -> Result<()> {
    let config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .map_err(|_| anyhow!("Unable to build config for logger"))?
        .build();

    match file {
        Some(file_path) => {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .context(format!("Unable to open logfile: {}", file_path))?;
            WriteLogger::init(LevelFilter::Info, config, file)
                .context("Unable to initialize logger for logfile")?;
        }
        None => {
            TermLogger::init(
                LevelFilter::Info,
                config,
                TerminalMode::Stdout,
                ColorChoice::Never,
            )
            .context("Unable to initialize logger for the terminal")?;
        }
    }
    Ok(())
}

pub fn save_ip(ip: &Ipv4Addr) -> Result<()> {
    let cache = cache_dir()?;
    if !cache.exists() {
        create_dir_all(&cache).context("Unable to create cache directory")?;
    }

    let bytes = wincode::serialize(ip).context("Unable to serialize IP")?;
    write(cache.join("ip"), bytes).context("Unable to save IP to file")?;
    Ok(())
}

pub fn read_ip() -> Result<Ipv4Addr> {
    let cache = cache_dir()?;
    let bytes = read(cache.join("ip")).context("IP cache does not exist")?;
    let ip: Ipv4Addr = wincode::deserialize(&bytes).context("Unable to read IP from file")?;
    Ok(ip)
}
