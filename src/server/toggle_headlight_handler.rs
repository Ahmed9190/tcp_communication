use crate::{
    server::command_enums::{SpeedMode, Turn},
    server::commands,
    server::handler::*,
    server::ClientMap,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ToggleHeadlightRequest {
    pub imei: String,
    pub state: bool, // `true` for on, `false` for off
}

#[derive(Serialize)]
pub struct ToggleHeadlightResponse {
    pub success: bool,
    pub message: String,
    pub imei: String,
}

pub async fn toggle_headlight_handler(
    State(clients): State<ClientMap>,
    Json(payload): Json<ToggleHeadlightRequest>,
) -> impl IntoResponse {
    let imei = payload.imei.clone();

    // Retrieve the client socket for the specified IMEI
    let mut socket = match get_client_socket(&clients, &imei).await {
        Ok(socket) => socket,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ToggleHeadlightResponse {
                    success: false,
                    message: err,
                    imei,
                }),
            );
        }
    };

    // Map the state to the Turn enum
    let headlight_switch = if payload.state { Turn::On } else { Turn::Off };

    // Set other parameters to 'Don't Set'
    let speed_mode = SpeedMode::DontSet;
    let throttle_response = Turn::DontSet;
    let taillights_flashing = Turn::DontSet;

    // Generate the S7 command
    let s7_command = commands::generate_s7_command(
        &imei,
        &headlight_switch,
        &speed_mode,
        &throttle_response,
        &taillights_flashing,
    );

    // Send the command to the scooter
    if let Err(err) = send_command(&mut socket, &s7_command).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ToggleHeadlightResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    // Handle the response from the scooter
    if let Err(err) = handle_s7_response(
        &mut socket,
        &imei,
        &headlight_switch,
        &speed_mode,
        &throttle_response,
        &taillights_flashing,
    )
    .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ToggleHeadlightResponse {
                success: false,
                message: err,
                imei,
            }),
        );
    }

    // Return a successful response
    (
        StatusCode::OK,
        Json(ToggleHeadlightResponse {
            success: true,
            message: "Headlight toggled successfully".to_string(),
            imei,
        }),
    )
}
