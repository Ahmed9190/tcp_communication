use handler::handle_connection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub mod commands;
pub mod handler;
pub mod lock_handler;
pub mod protocol;
pub mod scooter_command;
pub mod tests;
pub mod unlock_handler;

pub type ClientMap = Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>;

pub async fn start_server(address: &str, clients: ClientMap) -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind(address).await?;
    println!("Server running on {}", address);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        let clients = clients.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, clients).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
