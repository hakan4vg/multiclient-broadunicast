#!/bin/bash

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Build the project in release mode
echo "Building project..."
cargo build --release

# Create symbolic links or copy executables to a convenient location
echo "Installing executables..."
mkdir -p ~/.local/bin

# Copy the executables
cp target/release/multiclient-broadunicast ~/.local/bin/
cp target/release/server ~/.local/bin/
cp target/release/client ~/.local/bin/

echo "Installation complete!"
echo "You can now run the chat system with: multiclient-broadunicast OR by running run.sh" 