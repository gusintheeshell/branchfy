use crate::config::Config;
use rspotify::model::{PlayableItem, PlaylistId, Token};
use rspotify::prelude::*;
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use std::fs;
use std::io::{self, BufRead};

const TOKEN_FILE: &str = "spotify_token.json";

pub struct Player;

impl Player {
    pub async fn play_playlist(_config: &Config, playlist_id: &str) {
        async fn authenticate_spotify(
            spotify: &AuthCodeSpotify,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let url = spotify.get_authorize_url(false)?;
            println!("Please open this URL in your browser: {}\n", url);
            println!("After authenticating, paste the code from the URL here:");

            let stdin = io::stdin();
            let code = stdin.lock().lines().next().expect("Failed to read line")?;

            spotify.request_token(&code).await?;

            let token_ref = spotify.get_token();
            let token = {
                let token_lock = token_ref.lock().await;
                token_lock.unwrap().clone()
            };
            if let Some(token) = token.clone() {
                fs::write(TOKEN_FILE, serde_json::to_string(&token)?)?;
            } else {
                return Err("Failed to get access token".into());
            }

            Ok(())
        }

        async fn load_token(spotify: &AuthCodeSpotify) -> Result<bool, Box<dyn std::error::Error>> {
            if let Ok(token_data) = fs::read_to_string(TOKEN_FILE) {
                if let Ok(token) = serde_json::from_str::<Token>(&token_data) {
                    let mut token_guard = match spotify.token.lock().await {
                        Ok(guard) => guard,
                        Err(e) => {
                            return Err(<Box<dyn std::error::Error>>::from(format!(
                                "Failed to acquire lock: {:?}",
                                e
                            )))
                        }
                    };
                    token_guard.replace(token);
                    return Ok(true);
                }
            }
            Ok(false)
        }

        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!(
            "user-read-playback-state",
            "user-modify-playback-state"
        ))
        .unwrap();
        let spotify = AuthCodeSpotify::new(creds, oauth);

        if !load_token(&spotify).await.unwrap() {
            if let Err(e) = authenticate_spotify(&spotify).await {
                eprintln!("Error authenticating Spotify: {}", e);
                return;
            }
        }

        if spotify.refresh_token().await.is_err() {
            if let Err(e) = authenticate_spotify(&spotify).await {
                eprintln!("Error re-authenticating Spotify: {}", e);
                return;
            }
        }

        let devices = spotify.device().await.unwrap();
        if devices.is_empty() {
            println!("No active devices found. Open the Spotify app and try again.");
            return;
        } else {
            println!("Active devices:");
            for device in devices.iter() {
                println!(
                    "{} - {}",
                    device.id.as_deref().unwrap_or("Unknown ID"),
                    device.name
                );
            }
        }

        let playlist_id = PlaylistId::from_id(playlist_id).expect("Invalid playlist ID");
        let device_id = devices.get(0).and_then(|d| d.id.clone());

        if let Err(err) = spotify
            .start_context_playback(playlist_id.into(), device_id.as_deref(), None, None)
            .await
        {
            eprintln!("Error starting playback: {:?}", err);
            return;
        }

        if let Some(current_playback) = spotify.current_playback(None, None::<&[_]>).await.unwrap()
        {
            if let Some(item) = current_playback.item {
                match item {
                    PlayableItem::Track(track) => {
                        println!("Playing: {}", track.name);
                    }
                    PlayableItem::Episode(episode) => {
                        println!("Playing episode: {}", episode.name);
                    }
                }
            } else {
                println!("No track or episode is currently playing.");
            }
        } else {
            println!("No current playback.");
        }
    }
}
