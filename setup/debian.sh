#!/bin/bash
# Debian/Ubuntu Setup Script for Payment Plan Project

set -e

echo "Setting up development environment for Debian/Ubuntu..."

# Define required verions
REQUIRED_RUST_VERSION="1.81.0"
REQUIRED_NODE_VERSION="22.0.0"
REQUIRED_PHP_VERSION="8.1"

# Update system
sudo apt-get update
sudo apt-get upgrade -y

# Install base development tools
sudo apt-get install -y build-essential git curl wget ca-certificates gnupg lsb-release

# Function to compare version numbers
version_compare() {
    local version1=$1
    local version2=$2

    # Convert versions to comparable format (remove non-numeric suffixes)
    local v1=$(echo "$version1" | sed 's/[^0-9.].*//')
    local v2=$(echo "$version2" | sed 's/[^0-9.].*//')

    # Use sort -V for version comparison
    if [ "$(printf '%s\n' "$v1" "$v2" | sort -V | head -n1)" = "$v2" ]; then
        return 0  # version1 >= version2
    else
        return 1  # version1 < version2
    fi
}

# Install Rust
if command -v rustc &> /dev/null; then
    CURRENT_RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    if version_compare "$CURRENT_RUST_VERSION" "$REQUIRED_RUST_VERSION"; then
        echo "Rust is already installed with sufficient version ($(rustc --version))"
    else
        echo "Rust version $CURRENT_RUST_VERSION is installed but version $REQUIRED_RUST_VERSION or higher is required"
        echo "Updating Rust..."
        rustup update stable
        echo "Rust updated successfully"
    fi
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
    CURRENT_NODE_VERSION=$(node --version | sed 's/^v//')
    if version_compare "$CURRENT_NODE_VERSION" "$REQUIRED_NODE_VERSION"; then
        echo "Node.js is already installed with sufficient version ($(node --version))"
    else
        echo "Node.js version $CURRENT_NODE_VERSION is installed but version $REQUIRED_NODE_VERSION or higher is required"
        # Check if nvm is available
        if [ -f "$HOME/.nvm/nvm.sh" ] || command -v nvm &> /dev/null; then
            echo "Using nvm to update Node.js..."
            # Source nvm if it exists
            [ -f "$HOME/.nvm/nvm.sh" ] && source "$HOME/.nvm/nvm.sh"
            nvm install $REQUIRED_NODE_VERSION
            nvm use $REQUIRED_NODE_VERSION
            nvm alias default $REQUIRED_NODE_VERSION
            echo "Node.js updated successfully using nvm"
        else
            echo "nvm not found, updating Node.js via package manager..."
            curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
            sudo apt-get install -y nodejs
            echo "Node.js updated successfully"
        fi
    fi
else
    echo "Installing Node.js and npm..."
    # Check if nvm is available
    if [ -f "$HOME/.nvm/nvm.sh" ] || command -v nvm &> /dev/null; then
        echo "Using nvm to install Node.js..."
        # Source nvm if it exists
        [ -f "$HOME/.nvm/nvm.sh" ] && source "$HOME/.nvm/nvm.sh"
        nvm install $REQUIRED_NODE_VERSION
        nvm use $REQUIRED_NODE_VERSION
        nvm alias default $REQUIRED_NODE_VERSION
        echo "Node.js installed successfully using nvm"
    else
        echo "nvm not found, installing Node.js via package manager..."
        # Install NodeSource repository
        curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
        sudo apt-get install -y nodejs
        echo "Node.js and npm installed successfully"
    fi
fi

# Install Python
if command -v python3 &> /dev/null; then
    echo "Python is already installed ($(python3 --version))"
else
    echo "Installing Python..."
    sudo apt-get install -y python3 python3-pip python3-venv
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
    
    if [ "$JAVA_MAJOR" -ge 17 ]; then
        echo "Java is already installed with sufficient version ($(java -version 2>&1 | head -n1))"
    else
        echo "Java version $JAVA_VERSION is too old (minimum: Java 17)"
        echo "Installing OpenJDK 17..."
        sudo apt-get install -y openjdk-17-jdk
        echo "OpenJDK 17 installed successfully"
    fi
else
    echo "Installing Java (OpenJDK 17) for Kotlin development..."
    sudo apt-get install -y openjdk-17-jdk
    echo "OpenJDK 17 installed successfully"
fi

# Install PHP and extensions
if command -v php &> /dev/null; then
    CURRENT_PHP_VERSION=$(php -r "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION;")
    if version_compare "$CURRENT_PHP_VERSION" "$REQUIRED_PHP_VERSION"; then
        echo "PHP is already installed with sufficient version ($(php --version | head -n1))"
    else
        echo "PHP version $CURRENT_PHP_VERSION is installed but version $REQUIRED_PHP_VERSION or higher is required"
        echo "Please update your system or manually install PHP $REQUIRED_PHP_VERSION or higher"
        echo "Current PHP version is insufficient for this project"
        exit 1
    fi
else
    echo "Installing PHP..."

    # Install PHP from default repositories
    sudo apt-get install -y php php-cli php-common php-curl php-json php-mbstring php-xml php-bcmath

    # Check if the installed version meets requirements
    INSTALLED_PHP_VERSION=$(php -r "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION;")
    if version_compare "$INSTALLED_PHP_VERSION" "$REQUIRED_PHP_VERSION"; then
        echo "PHP installed successfully with version $INSTALLED_PHP_VERSION"
    else
        echo "Warning: Installed PHP version $INSTALLED_PHP_VERSION is below required version $REQUIRED_PHP_VERSION"
        echo "Please consider upgrading your system or manually installing a newer PHP version"
        echo "The project may not work correctly with this PHP version"
    fi
fi

# Install and enable FFI extension
if php -m | grep -q "FFI"; then
    echo "PHP FFI extension is already enabled"
else
    echo "Installing and enabling PHP FFI extension..."
    # Install FFI extension from default repositories
    sudo apt-get install -y php-ffi

    # Get the current PHP version being used
    CURRENT_PHP_VERSION=$(php -r "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION;")

    # Enable FFI extension
    FFI_INI="/etc/php/${CURRENT_PHP_VERSION}/cli/conf.d/20-ffi.ini"

    if [ -f "$FFI_INI" ]; then
        echo "FFI extension configuration found at $FFI_INI"
    else
        echo "extension=ffi" | sudo tee "$FFI_INI"
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

# Install cross-compilation tools for Windows
echo "Installing cross-compilation tools for Windows..."
sudo apt-get install -y mingw-w64

echo "Setup complete! You may need to restart your shell or run 'source ~/.cargo/env'"
echo "For Go commands, you may also need to run 'source ~/.bashrc' or restart your shell"
echo "To test the setup, run 'make test' in the project directory"
