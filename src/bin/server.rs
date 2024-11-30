use multiclient_broadunicast::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run_server(8080).await
}
