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
pub struct UnlockRequest {
    pub imei: String,
}

#[derive(Serialize)]
pub struct UnlockResponse {
    pub success: bool,
    pub message: String,
    pub imei: String,
}

pub async fn unlock_handler(
    State(clients): State<ClientMap>,
    Json(payload): Json<UnlockRequest>,
) -> impl IntoResponse {
    let imei = payload.imei.clone();
    let r0_operation = commands::R0Operation::Unlock;
    let r0_timestamp = timestamp::current();

    let mut socket = match get_client_socket(&clients, &imei).await {
        Ok(socket) => socket,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(UnlockResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let r0_command = commands::generate_r0_command(&imei, &r0_operation, 20, USER_ID, r0_timestamp);
    if let Err(err) = send_command(&mut socket, &r0_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UnlockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    let r0_key = match handle_r0_response(&mut socket, &imei, &r0_operation, r0_timestamp).await {
        Ok(key) => key,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UnlockResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let l0_timestamp = timestamp::current();
    let l0_command = commands::generate_l0_command(&imei, &r0_key, USER_ID, l0_timestamp);
    if let Err(err) = send_command(&mut socket, &l0_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UnlockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    if let Err(err) = handle_l_response(&mut socket, &imei, "L0", Some(l0_timestamp)).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UnlockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    let final_ack = commands::generate_l0_ack(&imei);
    if let Err(err) = send_command(&mut socket, &final_ack).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UnlockResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    (
        StatusCode::OK,
        Json(UnlockResponse {
            success: true,
            message: "Unlock operation completed successfully".to_string(),
            imei,
        }),
    )
}
