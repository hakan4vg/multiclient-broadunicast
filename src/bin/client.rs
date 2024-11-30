use clap::Parser;
use std::io::{self, Write};
use multiclient_broadunicast::client;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Prompt for username
    print!("Enter your username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    client::run_client(username, args.port).await
}
