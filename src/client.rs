use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncReadExt};
use chrono::Utc;
use crossterm::event::{read, Event, KeyCode};
use serde_json;

use crate::message::{Message, MessageTarget};

pub async fn run_client(username: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

    // Send username
    stream.write_all(username.as_bytes()).await?;
    // Spawn a task to receive messages
    let (read_half, mut write_half) = stream.into_split();
    tokio::spawn(async move {
        let mut reader = BufReader::new(read_half);
        let mut buffer = vec![0; 1024];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("\nDisconnected from server");
                    break;
                },
                Ok(n) => {
                    match serde_json::from_slice::<Message>(&buffer[..n]) {
                        Ok(message) => {
                            let delivery_time = Utc::now();
                            let delivery_duration = delivery_time - message.send_time;

                            println!(
                                "\n[Received Message]\n\
                                From: {}\n\
                                To: {}\n\
                                Message: {}\n\
                                Delivery Time: {:.2} ms",
                                message.sender,
                                match &message.target {
                                    MessageTarget::Broadcast => "All",
                                    MessageTarget::Unicast(target) => target,
                                },
                                message.content,
                                delivery_duration.num_milliseconds()
                            );
                        },
                        Err(e) => eprintln!("Error deserializing message: {}", e),
                    }
                },
                Err(_) => break,
            }
        }
    });

    // Main client interaction loop
    loop {
        println!("\n[Waiting Mode] Press any key to send a message...");

        // Wait for key press to enter sender mode
        read()?;

        // Send Mode
        println!("Enter message type (B for Broadcast, U for Unicast):");
        let message_type = match read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('b') | KeyCode::Char('B') => MessageTarget::Broadcast,
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    println!("Enter recipient's username:");
                    let mut recipient = String::new();
                    std::io::stdin().read_line(&mut recipient)?;
                    MessageTarget::Unicast(recipient.trim().to_string())
                },
                _ => continue,
            },
            _ => continue,
        };

        // Compose message
        println!("Enter your message:");
        let mut content = String::new();
        std::io::stdin().read_line(&mut content)?;

        // Create and send message
        let message = Message {
            sender: username.clone(),
            send_time: Utc::now(),
            target: message_type,
            content: content.trim().to_string(),
        };

        let serialized = serde_json::to_vec(&message)?;
        write_half.write_all(&serialized).await?;
    }
}