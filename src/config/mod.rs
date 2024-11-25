use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub playlists: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn load_or_create_global(
        config_path: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(content) = fs::read_to_string(config_path) {
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let mut playlists = std::collections::HashMap::new();
            playlists.insert(
                "feat".into(),
                Input::new()
                    .with_prompt("Playlist ID for feat")
                    .interact_text()?,
            );
            playlists.insert(
                "fix".into(),
                Input::new()
                    .with_prompt("Playlist ID for fix")
                    .interact_text()?,
            );

            let config = Config { playlists };
            fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
            Ok(config)
        }
    }

    pub fn get_playlist_for_branch(&self, branch: &str) -> Option<&String> {
        for (prefix, playlist) in &self.playlists {
            if branch.starts_with(prefix) {
                return Some(playlist);
            }
        }
        None
    }
}
