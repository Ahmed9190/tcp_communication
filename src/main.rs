use axum::{routing::post, Router};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use server::{
    change_gear_handler::change_gear_handler, change_headlight_handler::change_headlight_handler,
    lock_handler::lock_handler, start_server, unlock_handler::unlock_handler,
};

pub mod commands;
pub mod config;
pub mod errors;
pub mod logs;
pub mod server;
pub mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logs::init();

    let clients: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Start TCP server for main functionality
    let tcp_clients_main = clients.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(config::SERVER_ADDRESS, tcp_clients_main).await {
            eprintln!("Error in TCP server: {}", e);
        }
    });

    // Start second TCP listener for parsing
    let tcp_clients_parser = clients.clone();
    tokio::spawn(async move {
        let parser_address = "127.0.0.1:5000"; // Change as needed
        let parser_listener = TcpListener::bind(parser_address)
            .await
            .expect("Failed to bind parser listener");

        println!("Parser server listening on {}", parser_address);

        loop {
            match parser_listener.accept().await {
                Ok((stream, addr)) => {
                    println!("New parser connection from {}", addr);
                    let tcp_clients = tcp_clients_parser.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_parser_connection(stream, tcp_clients).await {
                            eprintln!("Error handling parser connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting parser connection: {}", e);
                }
            }
        }
    });

    // Start REST API server
    let app = Router::new()
        .route("/unlock", post(unlock_handler))
        .route("/lock", post(lock_handler))
        .route("/change-gear", post(change_gear_handler))
        .route("/change-headlight", post(change_headlight_handler))
        .with_state(clients);

    let listen_addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let listener = TcpListener::bind(listen_addr).await?;
    println!("HTTP server listening on {}", listen_addr);

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

// Function to handle parser connections
async fn handle_parser_connection(
    stream: TcpStream,
    clients: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];
    let mut stream = stream;

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break; // Connection closed
        }

        // Process the received data
        let received_data = String::from_utf8_lossy(&buf[..n]);
        println!("Received parser data: {}", received_data);

        // Implement parsing logic here
        // Optionally interact with `clients` if necessary
    }

    Ok(())
}
