use crate::config::Config;
use rspotify::model::{PlayableItem, PlaylistId};
use rspotify::prelude::*;
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use std::io::{self, BufRead};

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

            if spotify.get_token().lock().await.unwrap().is_none() {
                return Err("Failed to get access token".into());
            }

            Ok(())
        }

        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!(
            "user-read-playback-state",
            "user-modify-playback-state"
        ))
        .unwrap();
        let spotify = AuthCodeSpotify::new(creds, oauth);

        if let Err(e) = authenticate_spotify(&spotify).await {
            eprintln!("Error authenticating Spotify: {}", e);
            return;
        }

        let devices = spotify.device().await.unwrap();
        if devices.is_empty() {
            println!("Nenhum dispositivo ativo encontrado. Abra o aplicativo do Spotify e tente novamente.");
            return;
        } else {
            println!("Dispositivos ativos:");
            for device in devices.iter() {
                println!(
                    "{} - {}",
                    device.id.as_deref().unwrap_or("ID desconhecido"),
                    device.name
                );
            }
        }

        let playlist_id = PlaylistId::from_id(playlist_id).expect("ID da playlist inválido");
        let device_id = devices.get(0).and_then(|d| d.id.clone());

        if let Err(err) = spotify
            .start_context_playback(playlist_id.into(), device_id.as_deref(), None, None)
            .await
        {
            eprintln!("Erro ao iniciar a reprodução: {:?}", err);
            return;
        }

        if let Some(current_playback) = spotify.current_playback(None, None::<&[_]>).await.unwrap()
        {
            if let Some(item) = current_playback.item {
                match item {
                    PlayableItem::Track(track) => {
                        println!("Tocando: {}", track.name);
                    }
                    PlayableItem::Episode(episode) => {
                        println!("Tocando episódio: {}", episode.name);
                    }
                }
            } else {
                println!("Nenhuma música ou episódio sendo reproduzido.");
            }
        } else {
            println!("Não há reprodução atual.");
        }
    }
}
