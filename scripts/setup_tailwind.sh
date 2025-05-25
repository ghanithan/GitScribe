#!/bin/bash

TAILWIND_VERSION="3.4.1"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TAILWIND_DIR="$ROOT_DIR/.tailwindcss"
TAILWIND_BIN="$TAILWIND_DIR/tailwindcss"

# Create Tailwind directory if it doesn't exist
mkdir -p "$TAILWIND_DIR"

# Function to detect OS and architecture
detect_platform() {
    PLATFORM="$(uname -s)"
    ARCH="$(uname -m)"

    case "$PLATFORM" in
        Linux)
            PLATFORM="linux"
            ;;
        Darwin)
            PLATFORM="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            ;;
        *)
            echo "Unsupported platform: $PLATFORM"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x64"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            echo "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac

    echo "${PLATFORM}-${ARCH}"
}

# Download Tailwind CLI if it doesn't exist or if forced
download_tailwind() {
    if [ -f "$TAILWIND_BIN" ] && [ "$1" != "--force" ]; then
        echo "Tailwind CLI already exists at $TAILWIND_BIN"
        return 0
    fi

    PLATFORM=$(detect_platform)
    
    # Set the file extension based on platform
    if [[ "$PLATFORM" == *"windows"* ]]; then
        TAILWIND_BIN="${TAILWIND_BIN}.exe"
    fi

    DOWNLOAD_URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-${PLATFORM}"
    
    echo "Downloading Tailwind CSS v${TAILWIND_VERSION} for ${PLATFORM}..."
    curl -sL "$DOWNLOAD_URL" -o "$TAILWIND_BIN"
    chmod +x "$TAILWIND_BIN"
    
    echo "Tailwind CSS CLI downloaded to $TAILWIND_BIN"
}

# Run Tailwind with arguments
run_tailwind() {
    download_tailwind
    "$TAILWIND_BIN" "$@"
}

# Check if command is specified
if [ $# -eq 0 ]; then
    echo "Usage: $0 [build|watch|download]"
    exit 1
fi

# Process commands
case "$1" in
    download)
        download_tailwind "${@:2}"
        ;;
    build)
        run_tailwind -i "$ROOT_DIR/styles/input.css" -o "$ROOT_DIR/styles/output.css" --minify -c "$ROOT_DIR/tailwind.config.json"
        ;;
    watch)
        run_tailwind -i "$ROOT_DIR/styles/input.css" -o "$ROOT_DIR/styles/output.css" --watch -c "$ROOT_DIR/tailwind.config.json"
        ;;
    *)
        run_tailwind "$@"
        ;;
esac