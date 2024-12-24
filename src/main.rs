use axum::{routing::post, Router};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use server::{lock_handler::lock_handler, start_server, unlock_handler::unlock_handler};

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

    // Start TCP server
    let tcp_clients = clients.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(config::SERVER_ADDRESS, tcp_clients).await {
            eprintln!("Error in TCP server: {}", e);
        }
    });

    // Start REST API server
    let app = Router::new()
        .route("/unlock", post(unlock_handler))
        .route("/lock", post(lock_handler))
        .with_state(clients);

    let listen_addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(listen_addr).await?;

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}