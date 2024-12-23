# Multi-Client Chat Application

## Overview
# This is a multi-client CLI chat application consisting of server and N number of clients using Rust, Tokio, Serde, and Crossterm. Made for a school project by Hakan Avgın.
# https://github.com/hakan4vg/multiclient-broadunicast


## Features
- Supports N number of clients
- Broadcast and unicast messaging
- Cross-platform terminal launching
- Message delivery time calculation

## Requirements
- Windows, Linux, or macOS
- Internet connection for initial setup

## Setup and Running

## It can either be downloaded from the releases section or built from source.


### Automatic Installation
#### Windows
1. Download the repository
2. Double-click `build.bat`
   - This will automatically install Rust if needed
   - Build the project
   - Set up necessary paths

#### Linux/macOS
1. Download the repository
2. Open terminal in project directory
3. Make the build script executable:
```bash
chmod +x build.sh
```
4. Run the build script:
```bash
./build.sh
```

### Running the Application
#### Windows
- Double-click `run.bat` OR
- Open command prompt and run:
```bash
cargo run --bin multiclient_broadunicast
```

#### Linux/macOS
- Run `./run.sh` OR
- Open terminal and run:
```bash
cargo run --bin multiclient_broadunicast
```

### Messaging
- In each client window:
  - Press any key to enter sender mode
  - Choose message type:
    - 'b' for Broadcast (send to all clients)
    - 'u' for Unicast (send to specific client)
  - Enter message
  - Message delivery time will be displayed

## Technologies Used
- Rust
- Tokio (async runtime)
- Serde (serialization)
- Crossterm (CLI interaction)
