# This file is for building the app for release, not for running or testing the app. You can run the app with "cargo run".
FROM rust:latest
WORKDIR /app

# Install system dependencies for iced and cross-compilation
RUN apt-get update && apt-get install -y \
    # General build tools
    pkg-config \
    zip \
    # Linux GUI dependencies
    libx11-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libexpat1-dev \
    libssl-dev \
    libdbus-1-dev \
    # Windows cross-compilation
    gcc-mingw-w64-x86-64 \
    && rm -rf /var/lib/apt/lists/*

# Add Rust targets
RUN rustup target add \
    x86_64-unknown-linux-gnu \
    x86_64-pc-windows-gnu \
    aarch64-apple-darwin
 
# Create output directory
RUN mkdir -p /output
 
# Copy build script
COPY .build-docker.sh /usr/local/bin/build.sh
RUN chmod +x /usr/local/bin/build.sh
 
ENTRYPOINT ["/usr/local/bin/build.sh"]
