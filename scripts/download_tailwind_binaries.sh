#!/bin/bash

# This script downloads all platform-specific Tailwind CLI binaries
# Place them in the binaries directory for use by the application

TAILWIND_VERSION="3.4.1"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
BINARIES_DIR="$ROOT_DIR/binaries"

# Create binaries directory if it doesn't exist
mkdir -p "$BINARIES_DIR"

# List of platforms to download
PLATFORMS=(
    "linux-x64"
    "linux-arm64"
    "macos-x64"
    "macos-arm64"
    "windows-x64"
    "windows-arm64"
)

# Download each platform binary
for PLATFORM in "${PLATFORMS[@]}"; do
    BINARY_NAME="tailwindcss-$PLATFORM"
    if [[ "$PLATFORM" == *"windows"* ]]; then
        BINARY_NAME="$BINARY_NAME.exe"
    fi
    
    DOWNLOAD_URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-${PLATFORM}"
    OUTPUT_PATH="$BINARIES_DIR/$BINARY_NAME"
    
    echo "Downloading Tailwind CSS v${TAILWIND_VERSION} for ${PLATFORM}..."
    curl -sL "$DOWNLOAD_URL" -o "$OUTPUT_PATH"
    chmod +x "$OUTPUT_PATH"
    
    echo "Downloaded to $OUTPUT_PATH"
done

echo "All Tailwind CSS binaries downloaded successfully!"