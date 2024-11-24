use crate::config::Config;
use crate::git::get_current_branch;
use crate::player::Player;
use tokio::time::{sleep, Duration};

pub struct BranchWatcher {
    config: Config,
    current_branch: String,
    current_playlist: Option<String>,
}

impl BranchWatcher {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let current_branch = get_current_branch()?;
        Ok(Self {
            config,
            current_branch,
            current_playlist: None,
        })
    }

    pub async fn start(&mut self) {
        println!("Starting branch watcher...");

        self.check_and_play_playlist().await;

        loop {
            sleep(Duration::from_secs(5)).await;

            if let Ok(branch) = get_current_branch() {
                if branch != self.current_branch {
                    println!("Branch changed from {} to {}", self.current_branch, branch);
                    self.current_branch = branch;
                    self.check_and_play_playlist().await;
                }
            }
        }
    }

    async fn check_and_play_playlist(&mut self) {
        if let Some(playlist_id) = self.config.get_playlist_for_branch(&self.current_branch) {
            // Só troca se for uma playlist diferente
            if self.current_playlist.as_deref() != Some(playlist_id) {
                println!("Switching playlist for branch: {}", self.current_branch);
                Player::play_playlist(&self.config, playlist_id).await;
                self.current_playlist = Some(playlist_id.to_string());
            }
        }
    }
}
