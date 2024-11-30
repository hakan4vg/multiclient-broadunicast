# Multi-Client Chat Application

## Overview
This is a Rust-based client-server chat application that supports both broadcast and unicast messaging across multiple clients.

## Features
- Supports N number of clients
- Broadcast and unicast messaging
- Cross-platform terminal launching
- Message delivery time calculation

## Requirements
- Rust (latest stable version)
- Cargo package manager

## Setup and Running

### Installation
1. Ensure Rust and Cargo are installed
2. Clone the repository
3. Navigate to the project directory

### Running the Application
1. Start the main program:
```bash
cargo run
```
2. Enter the number of clients when prompted
3. The application will automatically launch:
   - One server terminal
   - N client terminals

### Messaging
- In each client window:
  - Press any key to enter sender mode
  - Choose message type:
    - 'B' for Broadcast (send to all clients)
    - 'U' for Unicast (send to specific client)
  - Enter message
  - Message delivery time will be displayed

## Technologies Used
- Rust
- Tokio (async runtime)
- Serde (serialization)
- Crossterm (CLI interaction)

## License
[Your License Here]
