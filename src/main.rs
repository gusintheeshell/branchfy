mod cli;
mod config;
mod daemon;
mod git;
mod player;
mod watcher;

use crate::cli::run as cli_run;
use crate::config::Config;
use crate::daemon::Daemon;
use directories::ProjectDirs;
use std::process::exit;

#[tokio::main]
async fn main() {
    cli_run();

    let proj_dirs = ProjectDirs::from("com", "branchfy", "branchfy")
        .expect("Failed to get project directories");

    let config_dir = proj_dirs.config_dir();
    std::fs::create_dir_all(config_dir).expect("Failed to create config directory");

    let config_path = config_dir.join("config.json");

    let config = Config::load_or_create_global(&config_path).unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        exit(1);
    });

    let daemon = Daemon::new(config);

    ctrlc::set_handler(move || {
        println!("Shutting down branchfy daemon...");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    daemon.start().await;
}
