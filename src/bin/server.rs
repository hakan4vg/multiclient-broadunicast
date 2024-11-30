use clap::Parser;
use multiclient_broadunicast::server;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    server::run_server(args.port).await
}
