#!/bin/bash
# Debian/Ubuntu Setup Script for Payment Plan Project

set -e

echo "Setting up development environment for Debian/Ubuntu..."

# Update system
sudo apt-get update
sudo apt-get upgrade -y

# Install base development tools
sudo apt-get install -y build-essential git curl wget ca-certificates gnupg lsb-release

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
if command -v uniffi-bindgen-go &> /dev/null; then
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
    # Remove any existing Go installation
    sudo rm -rf /usr/local/go

    # Download and install latest Go
    GO_VERSION=$(curl -s https://api.github.com/repos/golang/go/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
    GO_VERSION=${GO_VERSION#go}
    wget https://golang.org/dl/go${GO_VERSION}.linux-amd64.tar.gz
    sudo tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
    rm go${GO_VERSION}.linux-amd64.tar.gz

    # Add Go to PATH
    echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
    echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.zshrc 2>/dev/null || true
    export PATH=$PATH:/usr/local/go/bin
    echo "Go installed successfully"
fi

# Install Node.js and npm
if command -v node &> /dev/null; then
    echo "Node.js is already installed ($(node --version))"
else
    echo "Installing Node.js and npm..."
    # Install NodeSource repository
    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
    sudo apt-get install -y nodejs
    echo "Node.js and npm installed successfully"
fi

# Install Python
if command -v python3 &> /dev/null; then
    echo "Python is already installed ($(python3 --version))"
else
    echo "Installing Python..."
    sudo apt-get install -y python3 python3-pip python3-venv
    echo "Python installed successfully"
fi

# Install PHP and extensions
if command -v php &> /dev/null; then
    echo "PHP is already installed ($(php --version | head -n1))"
else
    echo "Installing PHP..."
    sudo apt-get install -y php php-cli php-common php-curl php-json php-mbstring php-xml php-bcmath
    echo "PHP installed successfully"
fi

# Install and enable FFI extension
if php -m | grep -q "FFI"; then
    echo "PHP FFI extension is already enabled"
else
    echo "Installing and enabling PHP FFI extension..."
    sudo apt-get install -y php-ffi

    # Enable FFI extension
    PHP_VERSION=$(php -r "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION;")
    FFI_INI="/etc/php/${PHP_VERSION}/cli/conf.d/20-ffi.ini"

    if [ -f "$FFI_INI" ]; then
        echo "FFI extension configuration found at $FFI_INI"
    else
        echo "extension=ffi" | sudo tee /etc/php/${PHP_VERSION}/cli/conf.d/20-ffi.ini
        echo "FFI extension enabled"
    fi
fi

# Install Composer
if command -v composer &> /dev/null; then
    echo "Composer is already installed ($(composer --version))"
else
    echo "Installing Composer..."
    sudo apt-get install -y composer
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

# Install Protocol Buffers compiler
echo "Installing Protocol Buffers compiler..."
sudo apt-get install -y protobuf-compiler

# Install Protocol Buffers Go plugin
if command -v go &> /dev/null; then
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
fi

# Install Playwright browser dependencies for Debian/Ubuntu
if command -v npx &> /dev/null; then
    echo "Installing Playwright browser dependencies for Debian/Ubuntu..."
    # Install system dependencies that Playwright needs
    sudo apt-get install -y \
        libnss3 \
        libnspr4 \
        libatk-bridge2.0-0 \
        libdrm2 \
        libxkbcommon0 \
        libxcomposite1 \
        libxdamage1 \
        libxrandr2 \
        libgbm1 \
        libxss1 \
        libgconf-2-4 \
        libxcursor1 \
        libxfixes3 \
        libxi6 \
        libxtst6 \
        libasound2 \
        libpangocairo-1.0-0 \
        libatk1.0-0 \
        libcairo-gobject2 \
        libgtk-3-0 \
        libgdk-pixbuf2.0-0 \
        fonts-liberation \
        xdg-utils \
        wget

    echo "Installing Playwright browsers..."
    echo "Playwright setup completed for Debian/Ubuntu"
fi

# Install cross-compilation tools for Windows
echo "Installing cross-compilation tools for Windows..."
sudo apt-get install -y mingw-w64

echo "Setup complete! You may need to restart your shell or run 'source ~/.cargo/env'"
echo "For Go commands, you may also need to run 'source ~/.bashrc' or restart your shell"
echo "To test the setup, run 'make test' in the project directory"
