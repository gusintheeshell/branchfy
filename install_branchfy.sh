#!/bin/bash

echo "Compiling the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Project compilation failed."
    exit 1
fi

echo "Copying the binary to /usr/local/bin..."
sudo cp target/release/branchfy /usr/local/bin/

if [ $? -ne 0 ]; then
    echo "Failed to copy the binary."
    exit 1
fi

echo "Setting up the BRANCHFY_PROJECT_DIR variable..."
PROFILE_PATH="$HOME/.zshrc"
if [ -f "$HOME/.bashrc" ]; then
    PROFILE_PATH="$HOME/.bashrc"
fi

BACKUP_PATH="${PROFILE_PATH}.backup"
cp "$PROFILE_PATH" "$BACKUP_PATH"

echo "export BRANCHFY_PROJECT_DIR=$(pwd)" >> "$PROFILE_PATH"

echo "Installation successful! You can use the 'branchfy' command."
echo "Backup of the profile file created at: $BACKUP_PATH"
