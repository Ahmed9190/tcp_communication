use serde::Deserialize;
use std::convert::TryFrom;
use tokio::io::AsyncWriteExt;

use super::{
    command_enums::{SpeedMode, Turn},
    ClientMap,
};

#[derive(Debug, Deserialize)]
pub enum R0Operation {
    Unlock,
    Lock,
    RFIDCardUnlock,
    RFIDCardLock,
}

impl TryFrom<&R0Operation> for &'static str {
    type Error = ();

    fn try_from(operation: &R0Operation) -> Result<Self, Self::Error> {
        match operation {
            R0Operation::Unlock => Ok("0"),
            R0Operation::Lock => Ok("1"),
            R0Operation::RFIDCardUnlock => Ok("2"),
            R0Operation::RFIDCardLock => Ok("3"),
        }
    }
}

impl<'a> TryFrom<&'a str> for R0Operation {
    type Error = &'a str;

    fn try_from(operation_string: &'a str) -> Result<Self, Self::Error> {
        match operation_string {
            "0" => Ok(R0Operation::Unlock),
            "1" => Ok(R0Operation::Lock),
            "2" => Ok(R0Operation::RFIDCardUnlock),
            "3" => Ok(R0Operation::RFIDCardLock),
            _ => Err(operation_string),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    Success,
    Failure,
    KeyError,
}
impl From<&str> for Status {
    fn from(status_string: &str) -> Self {
        match status_string {
            "0" => Status::Success,
            "1" => Status::Failure,
            "2" => Status::KeyError,
            _ => panic!("Invalid status code: {}", status_string),
        }
    }
}

pub fn generate_r0_command(
    imei: &str,
    operation: &R0Operation,
    key_duration: u8,
    user_id: u32,
    timestamp: i64,
) -> String {
    let operation: &str = operation.try_into().map_err(|_| ()).unwrap();
    generate_command(
        imei,
        "R0",
        &[
            &operation,
            &key_duration.to_string(),
            &user_id.to_string(),
            &timestamp.to_string(),
        ],
    )
}

pub fn generate_l0_command(imei: &str, key: &str, user_id: u32, timestamp: i64) -> String {
    generate_command(
        imei,
        "L0",
        &[key, &user_id.to_string(), &timestamp.to_string()],
    )
}

pub fn generate_l0_ack(imei: &str) -> String {
    generate_command(imei, "L0", &[])
}

pub fn generate_l1_command(imei: &str, key: &str) -> String {
    generate_command(imei, "L1", &[key])
}

pub fn generate_l1_ack(imei: &str) -> String {
    generate_command(imei, "L1", &[])
}

pub fn generate_s7_command(
    imei: &str,
    headlight: &Turn,
    speed_mode: &SpeedMode,
    throttle: &Turn,
    taillights_flashing: &Turn,
) -> String {
    generate_command(
        imei,
        "S7",
        &[
            &Into::<u8>::into(headlight).to_string(),
            &Into::<u8>::into(speed_mode).to_string(),
            &Into::<u8>::into(throttle).to_string(),
            &Into::<u8>::into(taillights_flashing).to_string(),
        ],
    )
}

pub fn generate_command(imei: &str, command: &str, content: &[&str]) -> String {
    let reserved_header = format!("{:#06X}", 0xFFFF);
    let vendor = crate::config::VENDOR;

    let mut command = format!(
        "{reserved_header}*SCOS,{vendor},{imei},{command}",
        reserved_header = reserved_header,
        vendor = vendor,
        imei = imei,
        command = command
    );
    if !content.is_empty() {
        command.push_str(",");
        command.push_str(&content.join(","));
    }
    command.push('#');
    command.push('\n');

    command
}

pub async fn send_command_to_imei(
    clients: ClientMap,
    imei: &str,
    command: &str,
) -> std::io::Result<()> {
    let clients_lock = clients.lock().await;

    if let Some(client) = clients_lock.get(imei) {
        let mut client_lock = client.lock().await; // Lock the TcpStream
        client_lock.write_all(command.as_bytes()).await?;
        println!("Command sent to {}: {}", imei, command);
    } else {
        println!("No client found with IMEI: {}", imei);
    }

    Ok(())
}
