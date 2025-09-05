use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use std::net::SocketAddr;

// Handle each client connection takes a TCP stream and the client’s address processes messages from the client.
async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    // Upgrade TCP stream to WebSocket
    let ws_stream = accept_async(stream).await;

    match ws_stream {
        Ok(mut ws) => {
            println!("WebSocket connection established with {}", addr);

            while let Some(msg) = ws.next().await {
                match msg {
                    Ok(msg) => {
                        println!("Received from {}: {:?}", addr, msg);

                        // Echo the message back
                        if ws.send(msg).await.is_err() {
                            println!("Error sending message to {}", addr);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("WebSocket error from {}: {}", addr, e);
                        break;
                    }
                }
            }
            println!("Connection closed: {}", addr);
        }
        Err(e) => eprintln!("Error during handshake with {}: {}", addr, e),
    }
}

// Tokio's async runtime attributed used for running the main function
#[tokio::main]
async fn main() {
    // Bind to local address and port
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(&addr).await.expect("Can't bind");

    println!("WebSocket server running on ws://{}", addr);

    // Accept incoming connections
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from {}", addr);

        // Spawn a new task for each connection: CALLING HANDLE_CONNECTION FUNCTION
        tokio::spawn(handle_connection(stream, addr));
    }
}

