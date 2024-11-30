use std::io::{self, Write};
use multiclient_broadunicast::terminal_launcher::launch_terminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _port = 8080; // Default port

    // Prompt for number of clients
    print!("Enter the number of clients to create: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let num_clients: u32 = input.trim().parse()?;

    // Launch server first
    launch_terminal("Chat Server", "./server")?;

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Launch clients
    for i in 1..=num_clients {
        launch_terminal(&format!("Chat Client {}", i), "./client")?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}