#!/bin/bash

set -euo pipefail

echo "Installing creatorly..."

# Download the latest release of your CLI tool
RELEASE_URL=$(curl --silent "https://api.github.com/repos/BuriKizilkaya/creatorly/releases/latest" | grep '"browser_download_url":' | sed -E 's/.*"([^"]+)".*/\1/')
curl -L -o creatorly "$RELEASE_URL"

# Make the CLI tool executable
chmod +x creatorly

# Move the CLI tool to /usr/local/bin so it's in the user's PATH
sudo mv creatorly /usr/local/bin

echo "creatorly installed!"

creatorly --version
