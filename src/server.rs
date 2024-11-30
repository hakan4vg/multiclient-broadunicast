use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use serde_json;
use std::collections::HashMap;
use std::process;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;

use crate::message::{Message, MessageTarget};

pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server listening on port {}", port);

    // Thread-safe map of connected clients
    let clients: Arc<Mutex<HashMap<String, OwnedWriteHalf>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _addr) = listener.accept().await?;
        let clients_clone = Arc::clone(&clients);

        let peer_addr = socket.peer_addr()?;

        let (read_half, write_half) = socket.into_split();
        let mut reader = BufReader::new(read_half);

        tokio::spawn(async move {
            // Read username first
            let mut username_buffer = [0; 1024];
            let username_len = match reader.read(&mut username_buffer).await {
                Ok(len) => len,
                Err(_) => return,
            };
            let username = String::from_utf8_lossy(&username_buffer[..username_len]).trim().to_string();

            println!("Client '{}' connected (Address: {}:{}, PID: {})", username, peer_addr.ip(), peer_addr.port(), process::id());

            {
                let mut clients = clients_clone.lock().await;
                clients.insert(username.clone(), write_half);
            }

            // Handle incoming messages
            let mut buffer = vec![0; 1024];
            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => {
                        // Client disconnected
                        let mut clients = clients_clone.lock().await;
                        clients.remove(&username);
                        println!("Client {} disconnected", username);
                        break;
                    },
                    Ok(n) => {
                        // Deserialize only the actual received bytes
                        match serde_json::from_slice::<Message>(&buffer[..n]) {
                            Ok(message) => {
                                let mut clients = clients_clone.lock().await;

                                match &message.target {
                                    MessageTarget::Broadcast => {
                                        for (target_username, client_socket) in clients.iter_mut() {
                                            if *target_username != message.sender {
                                                let serialized = serde_json::to_vec(&message).unwrap();
                                                let _ = client_socket.write_all(&serialized).await;
                                            }
                                        }
                                    },
                                    MessageTarget::Unicast(target) => {
                                        if let Some(client_socket) = clients.get_mut(target) {
                                            let serialized = serde_json::to_vec(&message).unwrap();
                                            let _ = client_socket.write_all(&serialized).await;
                                        }
                                    }
                                }
                            },
                            Err(e) => eprintln!("Error deserializing message: {}", e),
                        }
                    },
                    Err(_) => break,
                }
            }
        });
    }
}