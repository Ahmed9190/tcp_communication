use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidCommand(String),
    ClientNotFound(String),
    SocketError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
            AppError::ClientNotFound(msg) => write!(f, "Client not found: {}", msg),
            AppError::SocketError(msg) => write!(f, "Socket error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
