use crate::{
    server::ClientMap,
    server::{commands, handler::*},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use super::command_enums::{SpeedMode, Turn};

#[derive(Deserialize)]
pub struct ChangeGearRequest {
    pub imei: String,
    pub gear: u8,
}

#[derive(Serialize)]
pub struct ChangeGearResponse {
    pub success: bool,
    pub message: String,
    pub imei: String,
}

pub async fn change_gear_handler(
    State(clients): State<ClientMap>,
    Json(payload): Json<ChangeGearRequest>,
) -> impl IntoResponse {
    let imei = payload.imei.clone();

    let mut socket = match get_client_socket(&clients, &imei).await {
        Ok(socket) => socket,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ChangeGearResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let speed_mode = match SpeedMode::try_from(payload.gear) {
        Ok(speed_mode) => speed_mode,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ChangeGearResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    let headlight_switch = Turn::DontSet;
    let throttle = Turn::DontSet;
    let taillight_flashing = Turn::DontSet;

    let s7_command = commands::generate_s7_command(
        &imei,
        &headlight_switch,
        &speed_mode,
        &throttle,
        &taillight_flashing,
    );

    if let Err(err) = send_command(&mut socket, &s7_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ChangeGearResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    if let Err(err) = handle_s7_response(
        &mut socket,
        &imei,
        &headlight_switch,
        &speed_mode,
        &throttle,
        &taillight_flashing,
    )
    .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ChangeGearResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    (
        StatusCode::OK,
        Json(ChangeGearResponse {
            success: true,
            message: "Gear changed".to_string(),
            imei,
        }),
    )
}
