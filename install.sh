#!/bin/bash

# exit when any command fails
set -e

echo "Installing creatorly..."

# Download the latest release of your CLI tool
RELEASE_URL=$(curl --silent "https://api.github.com/repos/klab365/creatorly/releases/latest" | grep '"browser_download_url":' | sed -E 's/.*"([^"]+)".*/\1/')
curl --silent -L -o creatorly "$RELEASE_URL"

# Make the CLI tool executable
chmod +x creatorly

# Move the CLI tool to /usr/local/bin so it's in the user's PATH
sudo mv creatorly /usr/local/bin

if [ -f /usr/local/bin/creatorly ]; then
    echo "creatorly installed successfully"
    creatorly --version
else
    echo "creatorly installation failed"
    exit 1
fi
