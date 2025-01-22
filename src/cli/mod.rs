use clap::{App, Arg, SubCommand};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};

pub fn run() {
    let matches = App::new("Branchfy CLI")
        .version("1.0")
        .author("gusintheshell https://github.com/gusintheshell")
        .about("CLI for Branchfy setup and control")
        .subcommand(
            SubCommand::with_name("setup")
                .about("Setup Spotify API keys")
                .arg(
                    Arg::with_name("client_id")
                        .long("client-id")
                        .help("Spotify Client ID")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("client_secret")
                        .long("client-secret")
                        .help("Spotify Client Secret")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("redirect_uri")
                        .long("redirect-uri")
                        .help("Spotify Redirect URI")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("start").about("Start Branchfy").arg(
                Arg::with_name("verbose")
                    .long("verbose")
                    .help("Run in verbose mode"),
            ),
        )
        .subcommand(SubCommand::with_name("stop").about("Stop Branchfy"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("setup") {
        let client_id = matches.value_of("client_id").unwrap();
        let client_secret = matches.value_of("client_secret").unwrap();
        let redirect_uri = matches.value_of("redirect_uri").unwrap();

        env::set_var("RSPOTIFY_CLIENT_ID", client_id);
        env::set_var("RSPOTIFY_CLIENT_SECRET", client_secret);
        env::set_var("RSPOTIFY_REDIRECT_URI", redirect_uri);

        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        let profile = if shell.contains("zsh") {
            "~/.zshrc"
        } else {
            "~/.bashrc"
        };

        let profile_path = shellexpand::tilde(profile).to_string();
        let export_commands = format!(
            "\nexport RSPOTIFY_CLIENT_ID={}\nexport RSPOTIFY_CLIENT_SECRET={}\nexport RSPOTIFY_REDIRECT_URI={}\n",
            client_id, client_secret, redirect_uri
        );

        // Cria um backup do arquivo de perfil
        let backup_path = format!("{}.backup", profile_path);
        if Path::new(&profile_path).exists() {
            fs::copy(&profile_path, &backup_path).expect("Failed to create backup of profile file");
        }

        // Adiciona as novas variáveis de ambiente ao final do arquivo de perfil
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&profile_path)
            .expect("Failed to open profile file");
        writeln!(file, "{}", export_commands).expect("Failed to write to profile");

        println!(
            "Spotify API keys and redirect URI have been set up and saved to {}",
            profile
        );
        println!("Backup do arquivo de perfil criado em: {}", backup_path);
    } else if let Some(matches) = matches.subcommand_matches("start") {
        let verbose = matches.is_present("verbose");

        if verbose {
            println!("Starting Branchfy in verbose mode...");
            Command::new("cargo")
                .arg("run")
                .spawn()
                .expect("Failed to start Branchfy");
        } else {
            println!("Starting Branchfy in background...");
            Command::new("cargo")
                .arg("run")
                .spawn()
                .expect("Failed to start Branchfy");
        }
    } else if matches.subcommand_matches("stop").is_some() {
        println!("Stopping Branchfy...");
        // Implement logic to stop the running Branchfy process
        // This could involve killing a process or stopping a service
    } else {
        println!("No valid subcommand was provided. Use --help for more information.");
        exit(1);
    }
}
