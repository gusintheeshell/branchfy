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
        eprintln!("Erro ao carregar as configurações: {}", err);
        exit(1);
    });

    let branch = get_current_branch().unwrap_or_else(|err| {
        eprintln!("Erro ao detectar branch: {}", err);
        exit(1);
    });

    if let Some(prefix) = config.get_playlist_for_branch(&branch) {
        println!("Tocando playlist para o prefixo: {}", prefix);
        Player::play_playlist(&config, prefix).await;
    } else {
        println!("Nenhuma playlist associada a este prefixo de branch.");
    }

    ctrlc::set_handler(move || {
        println!("Encerrando player...");
        exit(0);
    })
    .expect("Erro ao definir handler de Ctrl+C");
}
