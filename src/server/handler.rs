use std::sync::Arc;

use crate::server::commands::R0Operation;
use crate::server::protocol;
use crate::{commands::parser::parse_command, config::USER_ID};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use super::ClientMap;

pub async fn handle_connection(mut socket: TcpStream, clients: ClientMap) -> std::io::Result<()> {
    let mut buffer = vec![0; 1024];

    // Read the initial message to register the client
    let n = socket.read(&mut buffer).await?;
    let initial_message = String::from_utf8_lossy(&buffer[..n]);
    println!("Received: {}", initial_message);

    // Extract IMEI from the initial message
    if let Some(imei) = extract_imei(&initial_message) {
        // Add to the global client map
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(imei.clone(), Arc::new(Mutex::new(socket)));
        println!("Client registered: {}", imei);

        let parsed_message = parse_command(&initial_message);
        println!("Parsed message: {:?}", parsed_message);
    } else {
        println!("Invalid initial message: {}", initial_message);
        return Ok(()); // Ignore the client if the message is invalid
    }

    // Continue handling client-specific logic...
    Ok(())
}

fn extract_imei(message: &str) -> Option<String> {
    // Regex to extract IMEI from the Q0 message
    let regex = regex::Regex::new(r"^\*SCOR,[^,]+,(\d{15}),Q0,").ok()?;
    regex
        .captures(message)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

pub async fn get_client_socket(
    clients: &ClientMap,
    imei: &str,
) -> Result<tokio::sync::OwnedMutexGuard<tokio::net::TcpStream>, String> {
    let clients_lock = clients.lock().await;

    if let Some(client) = clients_lock.get(imei) {
        // Clone the Arc to ensure the returned OwnedMutexGuard is valid
        let client_clone = client.clone();
        drop(clients_lock); // Explicitly drop clients_lock to avoid holding the lock longer than necessary
        Ok(client_clone.lock_owned().await)
    } else {
        Err(format!("Client with IMEI {} not found", imei))
    }
}

pub async fn send_command(socket: &mut tokio::net::TcpStream, command: &str) -> Result<(), String> {
    socket
        .write_all(command.as_bytes())
        .await
        .map_err(|_| format!("Failed to send command: {}", command))
}

pub async fn read_response(
    socket: &mut tokio::net::TcpStream,
    buffer: &mut [u8],
) -> Result<String, String> {
    let n = socket
        .read(buffer)
        .await
        .map_err(|_| "Failed to read response".to_string())?;
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

pub async fn handle_r0_response(
    socket: &mut tokio::net::TcpStream,
    imei: &str,
    r0_operation: &R0Operation,
    timestamp: i64,
) -> Result<String, String> {
    let mut buffer = vec![0; 1024];
    loop {
        let response = read_response(socket, &mut buffer).await?;
        match protocol::validate_r0_response(&response, imei, r0_operation, USER_ID, timestamp) {
            Ok(key) => return Ok(key),
            Err(err) => {
                println!("Ignored invalid R0 response: {} ({})", response, err);
            }
        }
    }
}

pub async fn handle_l_response(
    socket: &mut tokio::net::TcpStream,
    imei: &str,
    command: &str,
    timestamp: Option<i64>,
) -> Result<(), String> {
    let mut buffer = vec![0; 1024];
    loop {
        let response = read_response(socket, &mut buffer).await?;
        let validation_result = match command {
            "L0" => protocol::validate_l0_response(&response, imei, USER_ID, timestamp.unwrap()),
            "L1" => protocol::validate_l1_response(&response, imei, USER_ID),
            _ => Err("Unknown command type"),
        };

        match validation_result {
            Ok(_) => {
                println!("Valid {} response received: {}", command, response);
                return Ok(());
            }
            Err(err) => {
                println!(
                    "Ignored invalid {} response: {} ({})",
                    command, response, err
                );
            }
        }
    }
}
