#!/bin/bash
# Arch Linux Setup Script for Payment Plan Project

set -e

echo "Setting up development environment for Arch Linux..."

# Update system
sudo pacman -Syu --noconfirm

# Install base development tools
sudo pacman -S --noconfirm base-devel git curl wget

# Install Rust
if command -v rustc &> /dev/null; then
    echo "Rust is already installed ($(rustc --version))"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    echo "Rust installed successfully"
fi

# Add Windows target for cross-compilation
rustup target add x86_64-pc-windows-gnu

# Install uniffi-bindgen-go
if command -V uniffi-bindgen-go &> /dev/null; then
    echo "uniffi-bindgen-go is already installed ($(uniffi-bindgen-go --version))"
else
    echo "Installing uniffi-bindgen-go..."
    cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3
    echo "uniffi-bindgen-go installed successfully"
fi

# Install Go
if command -v go &> /dev/null; then
    echo "Go is already installed ($(go version))"
else
    echo "Installing Go..."
    sudo pacman -S --noconfirm go
    echo "Go installed successfully"
fi

# Install Node.js and npm
if command -v node &> /dev/null; then
    echo "Node.js is already installed ($(node --version))"
else
    echo "Installing Node.js and npm..."
    sudo pacman -S --noconfirm nodejs npm
    echo "Node.js and npm installed successfully"
fi

# Install Python
if command -v python3 &> /dev/null; then
    echo "Python is already installed ($(python3 --version))"
else
    echo "Installing Python..."
    sudo pacman -S --noconfirm python python-pip
    echo "Python installed successfully"
fi

# Install Java (OpenJDK) for Kotlin development
if command -v java &> /dev/null; then
    JAVA_VERSION=$(java -version 2>&1 | head -n1 | cut -d'"' -f2 | cut -d'.' -f1-2)
    if [[ "$JAVA_VERSION" == "1.8" ]]; then
        JAVA_MAJOR=8
    else
        JAVA_MAJOR=$(echo "$JAVA_VERSION" | cut -d'.' -f1)
    fi
    
    if [ "$JAVA_MAJOR" -ge 11 ]; then
        echo "Java is already installed with sufficient version ($(java -version 2>&1 | head -n1))"
    else
        echo "Java version $JAVA_VERSION is too old (minimum: Java 11)"
        echo "Installing OpenJDK 11..."
        sudo pacman -S --noconfirm jdk11-openjdk
        echo "OpenJDK 11 installed successfully"
    fi
else
    echo "Installing Java (OpenJDK 11) for Kotlin development..."
    sudo pacman -S --noconfirm jdk11-openjdk
    echo "OpenJDK 11 installed successfully"
fi

# Install PHP and Composer
if command -v php &> /dev/null; then
    echo "PHP is already installed ($(php --version | head -n1))"
else
    echo "Installing PHP..."
    sudo pacman -S --noconfirm php
    echo "PHP installed successfully"
fi

# Check and enable FFI extension
if php -m | grep -q "FFI"; then
    echo "PHP FFI extension is already enabled"
else
    echo "Enabling PHP FFI extension..."
    PHP_INI=$(php --ini | grep "Loaded Configuration File" | cut -d: -f2 | xargs)
    if [ -n "$PHP_INI" ] && [ -f "$PHP_INI" ]; then
        echo "extension=FFI" | sudo tee -a "$PHP_INI"
        echo "FFI extension added to $PHP_INI"
    else
        echo "Warning: Could not find php.ini file. You may need to manually add 'extension=FFI' to your php.ini"
    fi
fi

if command -v composer &> /dev/null; then
    echo "Composer is already installed ($(composer --version))"
else
    echo "Installing Composer..."
    sudo pacman -S --noconfirm php-composer
    echo "Composer installed successfully"
fi

# Install wasm-pack
if command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is already installed ($(wasm-pack --version))"
else
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    echo "wasm-pack installed successfully"
fi

# Install Protocol Buffers Go plugin
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

# Install Playwright browsers and dependencies for Arch
if command -v npx &> /dev/null; then
    echo "Installing Playwright browser dependencies for Arch..."
    # Install system dependencies that Playwright needs
    sudo pacman -S --noconfirm --needed \
        gtk3 \
        libxss \
        libxtst \
        libxrandr \
        alsa-lib \
        pango \
        atk \
        cairo \
        gdk-pixbuf2 \
        libxcomposite \
        libxcursor \
        libxdamage \
        libxfixes \
        libxi \
        libxrender \
        ca-certificates \
        ttf-liberation \
        nss \
        xdg-utils \
        wget

    echo "Installing Playwright browsers..."
    echo "Playwright setup completed for Arch Linux"
fi

# Install cross-compilation tools for Windows
sudo pacman -S --noconfirm mingw-w64-gcc

echo "Setup complete! You may need to restart your shell or run 'source ~/.cargo/env'"
echo "To test the setup, run 'make test' in the project directory"