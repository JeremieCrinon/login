#!/bin/bash
# This file is made to be copied inside the build Docker container and not to be runned directly
set -e
 
# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color
 
# Configuration
APP_NAME="${APP_NAME:-app}"
OUTPUT_DIR="/output"
ASSETS_DIRS="${ASSETS_DIRS:-translations}"  # Comma-separated list of asset directories to include
 
# Available targets
declare -A TARGETS=(
    ["linux"]="x86_64-unknown-linux-gnu"
    ["windows"]="x86_64-pc-windows-gnu"
    ["macos-arm"]="aarch64-apple-darwin"
)
 
declare -A TARGET_EXTENSIONS=(
    ["linux"]=""
    ["windows"]=".exe"
    ["macos-arm"]=""
)
 
declare -A TARGET_DISPLAY_NAMES=(
    ["linux"]="Linux x86_64"
    ["windows"]="Windows x86_64"
    ["macos-arm"]="macOS ARM64"
)
 
# Parse arguments
SELECTED_TARGETS=()
MACOS_ENABLED=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --macos|-m)
            MACOS_ENABLED=true
            shift
            ;;
        --target|-t)
            IFS=',' read -ra TARGETS_ARG <<< "$2"
            SELECTED_TARGETS+=("${TARGETS_ARG[@]}")
            shift 2
            ;;
        --list|-l)
            echo "Available targets:"
            for target in "${!TARGETS[@]}"; do
                echo "  - $target (${TARGET_DISPLAY_NAMES[$target]})"
            done
            exit 0
            ;;
        --help|-h)
            echo "Usage: build.sh [OPTIONS]"
            echo ""
            echo "If you want to build for MacOS, please add the -m parameter. This might not work if you are not on a MacOS machine"
            echo ""
            echo "Options:"
            echo "  -t, --target TARGET[,TARGET...]  Build only specified targets (comma-separated)"
            echo "  -l, --list                       List available targets"
            echo "  -h, --help                       Show this help"
            echo "  -m, --macos                      If you want to build the app for macOS, you need to add this parameter"
            echo ""
            echo "Environment variables:"
            echo "  APP_NAME      Name of the application (default: app)"
            echo "  ASSETS_DIRS   Comma-separated list of asset directories to include (default: translations)"
            echo ""
            echo "Examples:"
            echo "  build.sh                         Build all targets except MacOS"
            echo "  build.sh -m                      Build all targets including MacOS"
            echo "  build.sh -t linux                Build only Linux"
            echo "  build.sh -t linux,windows        Build Linux and Windows"
            echo "  build.sh -t linux,macos -m       Build Linux and MacOS"

            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done
 
# If no targets specified, build all (excluding macos unless -m is set)
if [ ${#SELECTED_TARGETS[@]} -eq 0 ]; then
    for target in "${!TARGETS[@]}"; do
        if [[ "$target" == "macos-arm" && "$MACOS_ENABLED" == false ]]; then
            continue
        fi
        SELECTED_TARGETS+=("$target")
    done
fi
 
# Validate selected targets
for target in "${SELECTED_TARGETS[@]}"; do
    if [[ ! -v "TARGETS[$target]" ]]; then
        echo -e "${RED}Error: Unknown target '$target'${NC}"
        echo "Use --list to see available targets"
        exit 1
    fi
    if [[ "$target" == "macos-arm" && "$MACOS_ENABLED" == false ]]; then
        echo -e "${RED}Error: macOS target requires the -m flag${NC}"
        exit 1
    fi
done
 
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Cross-Platform Build Script${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "App name: ${GREEN}$APP_NAME${NC}"
echo -e "Assets:   ${GREEN}$ASSETS_DIRS${NC}"
echo -e "Targets:  ${GREEN}${SELECTED_TARGETS[*]}${NC}"
echo ""
 
# Clean output directory
rm -rf "$OUTPUT_DIR"/*
 
# Build each target
SUCCESSFUL_BUILDS=()
FAILED_BUILDS=()
 
for target in "${SELECTED_TARGETS[@]}"; do
    RUST_TARGET="${TARGETS[$target]}"
    EXTENSION="${TARGET_EXTENSIONS[$target]}"
    DISPLAY_NAME="${TARGET_DISPLAY_NAMES[$target]}"
    
    echo -e "${YELLOW}----------------------------------------${NC}"
    echo -e "${YELLOW}Building for $DISPLAY_NAME...${NC}"
    echo -e "${YELLOW}----------------------------------------${NC}"
    
    # Build
    if cargo build --release --target "$RUST_TARGET"; then
        echo -e "${GREEN}Build successful for $DISPLAY_NAME${NC}"
        
        # Determine the actual target directory (zigbuild uses target without glibc version)
        if [[ "$target" == "linux" ]]; then
            ACTUAL_TARGET="x86_64-unknown-linux-gnu"
        else
            ACTUAL_TARGET="$RUST_TARGET"
        fi
        
        # Create package directory
        PACKAGE_DIR="$OUTPUT_DIR/${APP_NAME}-${target}"
        mkdir -p "$PACKAGE_DIR"
        
        # Copy binary
        BINARY_PATH="target/$ACTUAL_TARGET/release/${APP_NAME}${EXTENSION}"
        if [ -f "$BINARY_PATH" ]; then
            cp "$BINARY_PATH" "$PACKAGE_DIR/"
            echo -e "  Copied binary: ${GREEN}${APP_NAME}${EXTENSION}${NC}"
        else
            echo -e "${RED}  Binary not found at $BINARY_PATH${NC}"
            FAILED_BUILDS+=("$target")
            continue
        fi
        
        # Copy asset directories
        IFS=',' read -ra ASSETS_ARRAY <<< "$ASSETS_DIRS"
        for asset_dir in "${ASSETS_ARRAY[@]}"; do
            asset_dir=$(echo "$asset_dir" | xargs)  # Trim whitespace
            if [ -d "$asset_dir" ]; then
                cp -r "$asset_dir" "$PACKAGE_DIR/"
                echo -e "  Copied assets: ${GREEN}$asset_dir/${NC}"
            else
                echo -e "  ${YELLOW}Warning: Asset directory '$asset_dir' not found${NC}"
            fi
        done
        
        # Create zip
        cd "$OUTPUT_DIR"
        zip -r "${APP_NAME}-${target}.zip" "${APP_NAME}-${target}"
        rm -rf "${APP_NAME}-${target}"
        cd /app
        
        echo -e "${GREEN}Created: ${APP_NAME}-${target}.zip${NC}"
        SUCCESSFUL_BUILDS+=("$target")
    else
        echo -e "${RED}Build failed for $DISPLAY_NAME${NC}"
        FAILED_BUILDS+=("$target")
    fi
    
    echo ""
done
 
# Summary
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Build Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
 
if [ ${#SUCCESSFUL_BUILDS[@]} -gt 0 ]; then
    echo -e "${GREEN}Successful builds:${NC}"
    for target in "${SUCCESSFUL_BUILDS[@]}"; do
        echo -e "  ✓ ${TARGET_DISPLAY_NAMES[$target]} -> ${APP_NAME}-${target}.zip"
    done
fi
 
if [ ${#FAILED_BUILDS[@]} -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed builds:${NC}"
    for target in "${FAILED_BUILDS[@]}"; do
        echo -e "  ✗ ${TARGET_DISPLAY_NAMES[$target]}"
    done
fi
 
echo ""
echo -e "Output directory: ${BLUE}$OUTPUT_DIR${NC}"
 
# List output files
if [ "$(ls -A $OUTPUT_DIR 2>/dev/null)" ]; then
    echo ""
    echo "Generated files:"
    ls -lh "$OUTPUT_DIR"/*.zip 2>/dev/null || true
fi
 
# Exit with error if any builds failed
if [ ${#FAILED_BUILDS[@]} -gt 0 ]; then
    exit 1
fi
