use clap::Parser;
mod client;
mod server;
mod message;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long)]
    username: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.username {
        Some(username) => client::run_client(username, args.port).await,
        None => server::run_server(args.port).await,
    }
}