use std::convert::TryFrom;

use crate::commands::{
    alarm_command::AlarmType,
    beep_command::BeepPlayContent,
    hearbeat_command::{ChargingStatus, ScooterStatus},
    parser::{parse_coordinates, parse_datetime},
    positioning_command::{self, Hemisphere, PositioningIdentifier, PositioningResponse},
    scooter_setting_command::{HeadlightSwitch, ModeSetting, TaillightsFlashing, ThrottleResponse},
};

use super::commands::{R0Operation, Status};

#[derive(Debug)]
pub enum ScooterCommand {
    UnlockOrLockResponse {
        imei: String,
        operation: R0Operation,
        key: u8, // Randomly generated key
        user_id: String,
        timestamp: String,
    },
    UnlockResponse {
        imei: String,
        status: Status,
        user_id: String,
        timestamp: String,
    },
    LockResponse {
        imei: String,
        status: Status,
        user_id: String,
        timestamp: String,
        cycling_time: u32,
    },
    PositioningResponse(positioning_command::PositioningResponse),
    AlarmCommand {
        imei: String,
        alarm_type: AlarmType,
    },
    BeepPlaybackCommand {
        imei: String,
        play_content: BeepPlayContent,
    },
    ScooterSetting {
        imei: String,
        headlight_switch: HeadlightSwitch,
        mode_setting: ModeSetting,
        throttle_response: ThrottleResponse,
        taillights_flashing: TaillightsFlashing,
    },
    SigningIn {
        imei: String,
        voltage: f32,
        power: u8,
        signal: u8,
    },
    HeartBeat {
        imei: String,
        status: ScooterStatus,
        voltage: f32,
        signal: u8,
        power: u8,
        charging: ChargingStatus,
    },
}

impl TryFrom<&[&str]> for ScooterCommand {
    type Error = String;

    fn try_from(parts: &[&str]) -> Result<Self, Self::Error> {
        if parts.len() < 5 {
            return Err("Invalid command format: insufficient parts".to_string());
        }

        let imei = parts[2].to_string();
        let command = parts[3];

        match command {
            "R0" => {
                if parts.len() < 8 {
                    return Err("Invalid R0 response: insufficient parts".to_string());
                }

                let operation: R0Operation =
                    parts[4].try_into().map_err(|_| "Invalid operation field")?;
                let key = parts[5]
                    .parse::<u8>()
                    .map_err(|_| "Invalid key field".to_string())?;
                let user_id = parts[6].to_string();
                let timestamp = parts[7].to_string();

                Ok(ScooterCommand::UnlockOrLockResponse {
                    imei,
                    operation,
                    key,
                    user_id,
                    timestamp,
                })
            }
            "L0" => {
                if parts.len() < 7 {
                    return Err("Invalid L0 response: insufficient parts".to_string());
                }

                let status: Status = Into::<Status>::into(parts[4]);
                let user_id = parts[5].to_string();
                let timestamp = parts[6].to_string();

                Ok(ScooterCommand::UnlockResponse {
                    imei,
                    status,
                    user_id,
                    timestamp,
                })
            }
            "L1" => {
                if parts.len() < 8 {
                    return Err("Invalid L1 response: insufficient parts".to_string());
                }

                let status = Into::<Status>::into(parts[4]);
                let user_id = parts[5].to_string();
                let timestamp = parts[6].to_string();
                let cycling_time = parts[7]
                    .parse::<u32>()
                    .map_err(|_| "Invalid cycling time field".to_string())?;

                Ok(ScooterCommand::LockResponse {
                    imei,
                    status,
                    user_id,
                    timestamp,
                    cycling_time,
                })
            }
            "D0" => {
                if parts.len() < 17 {
                    return Err("Invalid D0 response: insufficient parts".to_string());
                }

                let identifier: PositioningIdentifier = parts[4]
                    .parse::<u8>()
                    .map_err(|_| "Invalid positioning identifier")?
                    .try_into()?;

                let utc_time = parts[5].split('.').next().unwrap_or(&parts[5][0..5]);
                let positioning_status = parts[6]
                    .chars()
                    .next()
                    .ok_or("Invalid positioning status")?
                    .try_into()?;

                let latitude = parse_coordinates(parts[7], parts[8])?;
                let longitude = parse_coordinates(parts[9], parts[10])?;

                let satellites_number = parts[11]
                    .parse::<u8>()
                    .map_err(|_| "Invalid satellites number")?;
                let positioning_accuracy = parts[12]
                    .parse::<f32>()
                    .map_err(|_| "Invalid positioning accuracy")?;

                let utc_date = parts[13].split('.').next().unwrap_or(&parts[5][0..5]);
                let utc_datetime = parse_datetime(utc_time, utc_date)?;

                let altitude = parts[14].parse::<f32>().map_err(|_| "Invalid altitude")?;

                // Validate height unit
                if parts[15] != "M" {
                    return Err(format!("Invalid height unit: {}", parts[15]));
                }

                let mode = parts[16].chars().next().ok_or("Invalid mode")?.try_into()?;

                let latitude_hemisphere: Hemisphere = parts[8].try_into()?;
                let longitude_hemisphere: Hemisphere = parts[10].try_into()?;

                Ok(ScooterCommand::PositioningResponse(PositioningResponse {
                    imei,
                    identifier,
                    utc_datetime,
                    positioning_status,
                    latitude,
                    latitude_hemisphere,
                    longitude,
                    longitude_hemisphere,
                    satellites_number,
                    positioning_accuracy,
                    altitude,
                    mode,
                }))
            }
            "W0" => {
                if parts.len() < 5 {
                    return Err("Invalid W0 response: insufficient parts".to_string());
                }

                let imei = parts[2].to_string();
                let alarm_type: AlarmType = parts[4]
                    .parse::<u8>()
                    .map_err(|_| "Invalid alarm type")?
                    .try_into()?;

                Ok(ScooterCommand::AlarmCommand { imei, alarm_type })
            }
            "V0" => {
                if parts.len() < 5 {
                    return Err("Invalid V0 response: insufficient parts".to_string());
                }

                let imei = parts[2].to_string();
                let play_content: BeepPlayContent = parts[4]
                    .parse::<u8>()
                    .map_err(|_| "Invalid play content")?
                    .try_into()?;

                Ok(ScooterCommand::BeepPlaybackCommand { imei, play_content })
            }
            "S7" => {
                if parts.len() < 8 {
                    return Err("Invalid S7 response: insufficient parts".to_string());
                }

                let imei = parts[2].to_string();

                let headlight_switch: HeadlightSwitch = parts[4]
                    .parse::<u8>()
                    .map_err(|_| "Invalid headlight switch value")?
                    .try_into()?;

                let mode_setting: ModeSetting = parts[5]
                    .parse::<u8>()
                    .map_err(|_| "Invalid mode setting value")?
                    .try_into()?;

                let throttle_response: ThrottleResponse = parts[6]
                    .parse::<u8>()
                    .map_err(|_| "Invalid throttle response value")?
                    .try_into()?;

                let taillights_flashing: TaillightsFlashing = parts[7]
                    .parse::<u8>()
                    .map_err(|_| "Invalid taillights flashing value")?
                    .try_into()?;

                Ok(ScooterCommand::ScooterSetting {
                    imei,
                    headlight_switch,
                    mode_setting,
                    throttle_response,
                    taillights_flashing,
                })
            }
            "Q0" => {
                if parts.len() != 7 {
                    return Err("Invalid Q0 response: insufficient parts".to_string());
                }

                let imei = parts[2].to_string();
                let voltage = parts[4]
                    .parse::<f32>()
                    .map(|v| v / 100.0)
                    .map_err(|_| "Invalid voltage")?;
                let power = parts[5].parse::<u8>().map_err(|_| "Invalid power")?;
                let signal = parts[6].parse::<u8>().map_err(|_| "Invalid signal")?;

                Ok(ScooterCommand::SigningIn {
                    imei,
                    voltage,
                    power,
                    signal,
                })
            }
            "H0" => {
                if parts.len() != 9 {
                    return Err("Invalid H0 response: insufficient parts".to_string());
                }

                let imei = parts[2].to_string();
                let status: ScooterStatus = parts[4]
                    .parse::<u8>()
                    .map_err(|_| "Invalid scooter status")?
                    .try_into()?;
                let voltage = parts[5]
                    .parse::<f32>()
                    .map(|v| v / 100.0)
                    .map_err(|_| "Invalid voltage")?;
                let signal = parts[6].parse::<u8>().map_err(|_| "Invalid signal")?;
                let power = parts[7].parse::<u8>().map_err(|_| "Invalid power")?;
                let charging: ChargingStatus = parts[8]
                    .parse::<u8>()
                    .map_err(|_| "Invalid charging status")?
                    .try_into()?;

                Ok(ScooterCommand::HeartBeat {
                    imei,
                    status,
                    voltage,
                    signal,
                    power,
                    charging,
                })
            }

            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}
