use crate::{
    config::USER_ID,
    server::ClientMap,
    server::{commands, handler::*},
    utils::timestamp,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LockRequest {
    pub imei: String,
}

#[derive(Serialize)]
pub struct LockResponse {
    pub success: bool,
    pub message: String,
    pub imei: String,
}

pub async fn lock_handler(
    State(clients): State<ClientMap>,
    Json(payload): Json<LockRequest>,
) -> impl IntoResponse {
    let imei = payload.imei.clone();
    let r0_operation = commands::R0Operation::Lock;
    let timestamp = timestamp::current();

    let mut socket = match get_client_socket(&clients, &imei).await {
        Ok(socket) => socket,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(LockResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let r0_command = commands::generate_r0_command(&imei, &r0_operation, 20, USER_ID, timestamp);
    if let Err(err) = send_command(&mut socket, &r0_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    let r0_key = match handle_r0_response(&mut socket, &imei, &r0_operation, timestamp).await {
        Ok(key) => key,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LockResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let l1_command = commands::generate_l1_command(&imei, &r0_key);
    if let Err(err) = send_command(&mut socket, &l1_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    if let Err(err) = handle_l_response(&mut socket, &imei, "L1", None).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    let final_ack = commands::generate_l1_ack(&imei);
    if let Err(err) = send_command(&mut socket, &final_ack).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    (
        StatusCode::OK,
        Json(LockResponse {
            success: true,
            message: "Lock operation completed successfully".to_string(),
            imei,
        }),
    )
}
