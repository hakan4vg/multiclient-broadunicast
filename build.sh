#!/bin/bash

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    # Check if curl is installed
    if ! command -v curl &> /dev/null; then
        if command -v apt &> /dev/null; then
            sudo apt update && sudo apt install -y curl
        elif command -v yum &> /dev/null; then
            sudo yum install -y curl
        elif command -v brew &> /dev/null; then
            brew install curl
        else
            echo "Please install curl first"
            exit 1
        fi
    fi
    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Build the project in release mode
echo "Building project..."
cargo build --release

# Create installation directory
echo "Installing executables..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    install_dir="$HOME/Applications/multiclient_chat"
else
    # Linux
    install_dir="$HOME/.local/bin"
fi

mkdir -p "$install_dir"

# Copy the executables
cp target/release/multiclient_broadunicast "$install_dir/"
cp target/release/server "$install_dir/"
cp target/release/client "$install_dir/"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$install_dir:"* ]]; then
    echo "export PATH=\"\$PATH:$install_dir\"" >> ~/.bashrc
    echo "export PATH=\"\$PATH:$install_dir\"" >> ~/.profile
    if [ -f ~/.zshrc ]; then
        echo "export PATH=\"\$PATH:$install_dir\"" >> ~/.zshrc
    fi
fi

echo "Installation complete!"
echo "Please restart your terminal or run: source ~/.bashrc"
echo "You can now run the chat system with: multiclient_broadunicast on your terminal OR by running run.sh" 