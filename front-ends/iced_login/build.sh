#!/bin/bash
# Cross-platform build script using Docker
# Usage: ./cross-build.sh [OPTIONS]
#
# Options:
#   -t, --target TARGET[,TARGET...]  Build only specified targets
#   -m, --macos                      Enable macOS target (may not work outside macOS)
#   -l, --list                       List available targets
#   -h, --help                       Show help
#
# Examples:
#   ./cross-build.sh                 Build all targets except macOS
#   ./cross-build.sh -m              Build all targets including macOS
#   ./cross-build.sh -t linux        Build only Linux
#   ./cross-build.sh -t linux,windows Build Linux and Windows
#   ./cross-build.sh -t linux,macos-arm -m  Build Linux and macOS
 
set -e
 
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
 
# Configuration - Edit these for your project
APP_NAME=$(grep '^name' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')  # Extracted from Cargo.toml
ASSETS_DIRS="translations"       # Comma-separated list of directories to include
OUTPUT_DIR="./dist"              # Where to put the zip files
 
# Docker image name
IMAGE_NAME="rust-cross-builder"
 
# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed or not in PATH"
    exit 1
fi
 
# Build the Docker image if it doesn't exist or Dockerfile changed
DOCKERFILE_HASH=$(md5sum Dockerfile 2>/dev/null | cut -d' ' -f1 || md5 -q Dockerfile 2>/dev/null)
STORED_HASH=""
if [ -f ".dockerfile-hash" ]; then
    STORED_HASH=$(cat .dockerfile-hash)
fi
 
if [ "$DOCKERFILE_HASH" != "$STORED_HASH" ] || ! docker image inspect "$IMAGE_NAME" &> /dev/null; then
    echo "Building Docker image..."
    docker build -t "$IMAGE_NAME" -f Dockerfile .
    echo "$DOCKERFILE_HASH" > .dockerfile-hash
fi
 
# Create output directory
mkdir -p "$OUTPUT_DIR"
 
# Run the build
echo "Starting cross-platform build..."
docker run --rm \
    -v "$(pwd)":/app \
    -v "$(pwd)/$OUTPUT_DIR":/output \
    -v cargo-cache:/usr/local/cargo/registry \
    -e APP_NAME="$APP_NAME" \
    -e ASSETS_DIRS="$ASSETS_DIRS" \
    "$IMAGE_NAME" "$@"
 
echo ""
echo "Build complete! Check $OUTPUT_DIR for the output files."
