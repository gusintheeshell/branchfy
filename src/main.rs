mod config;
mod git;
mod player;
mod watcher;

use crate::config::Config;
use std::process::exit;
use watcher::BranchWatcher;

#[tokio::main]
async fn main() {
    let config = Config::load_or_create().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        exit(1);
    });

    let mut watcher = BranchWatcher::new(config).unwrap_or_else(|err| {
        eprintln!("Error creating watcher: {}", err);
        exit(1);
    });

    ctrlc::set_handler(move || {
        println!("Shutting down player...");
        exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    watcher.start().await;
}
