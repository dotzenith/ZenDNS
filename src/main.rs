mod providers;
mod common;
use crate::providers::CloudflareManager;

fn main() {
    println!("{}", common::get_ip().unwrap());
    let manager = CloudflareManager::new().unwrap();
    manager.update_dns_record().unwrap();
}
