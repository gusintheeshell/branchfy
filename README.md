# Branchfy 🎵

A Rust application that plays different Spotify playlists based on the prefix of your current git branch.

## Features

- 🎧 Automatically plays playlists based on your branch
- 🔄 Changes playlist when you switch branches
- ⚡ Keeps the music playing in the background
- 🎮 Controls Spotify via the official API

## Prerequisites

- Rust installed
- Spotify Premium account
- Spotify app active on some device
- Spotify API credentials

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/branchfy
cd branchfy

# Configure your Spotify credentials
cp .env.example .env
# Edit .env with your credentials
```

## Configuration

1. Create an application in the Spotify Developer Dashboard
2. Add `http://localhost/` as a redirect URI
3. Copy Client ID and Client Secret to the `.env` file

## Usage

```bash
# First run - will ask for playlist IDs
cargo run

# Playlist IDs can be found in the Spotify URL:
# spotify:playlist:37i9dQZF1DX5Ejj0EkURtP
#                   ^^^^^^^^^^^^^^^^^^^^^ This is the ID
```

## Branch Mapping

By default:

- `feat/*` -> Playlist configured for features
- `fix/*` -> Playlist configured for fixes

## Commands

- `Ctrl+C` - Stops the execution
- The music keeps playing until you explicitly stop it

## Structure

.
├── src/
│ ├── main.rs # Entry point
│ ├── config.rs # Config management
│ ├── git.rs # Git interaction
│ ├── player.rs # Spotify control
│ └── watcher.rs # Branch monitor
├── Cargo.toml # Dependencies
└── config.json # Playlist configuration

## Contribution

Pull requests are welcome! For major changes, please open an issue first.

## License

[MIT](https://choosealicense.com/licenses/mit/)
