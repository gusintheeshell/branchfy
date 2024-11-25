use crate::config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use rspotify::model::{PlayableItem, PlaylistId, Token};
use rspotify::prelude::*;
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use std::fs;
use std::io::{self, BufRead};
use std::time::Duration;

const TOKEN_FILE: &str = "spotify_token.json";

pub struct Player;

impl Player {
    fn format_duration(ms: u32) -> String {
        let total_seconds = ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    async fn display_progress(
        spotify: &AuthCodeSpotify,
        track_name: String,
        progress_ms: u64,
        duration_ms: u32,
    ) {
        let pb = ProgressBar::new(duration_ms as u64);
        pb.enable_steady_tick(Duration::from_millis(100));

        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {msg}\n{bar:40.cyan/blue} {pos_prefix}/{len_prefix}")
                .unwrap()
                .with_key(
                    "pos_prefix",
                    |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                        write!(w, "{}", Self::format_duration(state.pos() as u32)).unwrap()
                    },
                )
                .with_key(
                    "len_prefix",
                    |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                        write!(
                            w,
                            "{}",
                            Self::format_duration(state.len().unwrap_or(0) as u32)
                        )
                        .unwrap()
                    },
                )
                .progress_chars("=>-"),
        );

        pb.set_message(format!("Playing: {}", track_name));
        pb.set_position(progress_ms);

        loop {
            if let Ok(Some(playback)) = spotify.current_playback(None, None::<&[_]>).await {
                if let Some(progress_ms) = playback.progress {
                    // Changed from progress to progress_ms
                    pb.set_position(progress_ms.num_milliseconds() as u64);

                    // Format progress/total time
                    let current = Self::format_duration(progress_ms.num_milliseconds() as u32);
                    let total = Self::format_duration(duration_ms);
                    pb.set_message(format!("Playing: {} ({}/{})", track_name, current, total));

                    if progress_ms >= chrono::Duration::milliseconds(duration_ms as i64) {
                        pb.finish_with_message(format!("Finished: {} ({})", track_name, total));
                        break;
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

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

            if let Some(current_playback) =
                spotify.current_playback(None, None::<&[_]>).await.unwrap()
            {
                if let Some(item) = current_playback.item {
                    match item {
                        PlayableItem::Track(track) => {
                            if let (Some(progress), Some(duration)) =
                                (current_playback.progress, Some(track.duration))
                            {
                                let progress_ms = progress.num_milliseconds() as u32;
                                let duration_ms = duration.num_milliseconds() as u32;
                                Player::display_progress(
                                    &spotify,
                                    track.name.clone(),
                                    progress_ms.into(),
                                    duration_ms,
                                )
                                .await;
                            }
                        }
                        PlayableItem::Episode(episode) => {
                            println!("Playing episode: {}", episode.name)
                        }
                    }
                }
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
