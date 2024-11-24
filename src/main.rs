mod config;
mod git;
mod player;

use crate::config::Config;
use crate::git::get_current_branch;
use crate::player::Player;
use std::process::exit;

#[tokio::main]
async fn main() {
    let config = Config::load_or_create().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        exit(1);
    });

    let branch = get_current_branch().unwrap_or_else(|err| {
        eprintln!("Error detecting branch: {}", err);
        exit(1);
    });

    if let Some(prefix) = config.get_playlist_for_branch(&branch) {
        println!("Playing playlist for prefix: {}", prefix);
        Player::play_playlist(&config, prefix).await;
    } else {
        println!("No playlist associated with this branch prefix.");
    }

    ctrlc::set_handler(move || {
        println!("Shutting down player...");
        exit(0);
    })
    .expect("Error setting Ctrl+C handler");
}
