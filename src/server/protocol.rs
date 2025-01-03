use regex::Regex;

use super::commands::R0Operation;

pub fn validate_r0_response(
    response: &str,
    imei: &str,
    r0_operation: &R0Operation,
    user_id: u32,
    timestamp: i64,
) -> Result<String, &'static str> {
    match validate_command(
        response,
        imei,
        "R0",
        &[
            r0_operation
                .try_into()
                .map_err(|_| "Invalid R0 operation")?,
            &r"\d+",
            &user_id.to_string(),
            &timestamp.to_string(),
        ],
    ) {
        Ok(_) => {
            let key = response
                .split(',')
                .nth(5)
                .ok_or("Failed to extract key from response")?;
            Ok(key.to_string())
        }
        Err(err) => Err(err),
    }
}

pub fn validate_l0_response(
    response: &str,
    imei: &str,
    user_id: u32,
    timestamp: i64,
) -> Result<(), &'static str> {
    validate_command(
        response,
        imei,
        "L0",
        &[&"0", &user_id.to_string(), &timestamp.to_string()],
    )
}

pub fn validate_l1_response(response: &str, imei: &str, user_id: u32) -> Result<(), &'static str> {
    validate_command(
        response,
        imei,
        "L1",
        &[
            &"0", // Expect "0" (success)
            &user_id.to_string(),
            r"\d+", // Unlock timestamp
            r"\d+", // Cycling time
        ],
    )
}

pub fn validate_s7_response(
    response: &str,
    imei: &str,
    headlight_switch: u8,
    mode_setting: u8,
    throttle_response: u8,
    taillights_flashing: u8,
) -> Result<(), &'static str> {
    validate_command(
        response,
        imei,
        "S7",
        &[
            &headlight_switch.to_string(),
            &mode_setting.to_string(),
            &throttle_response.to_string(),
            &taillights_flashing.to_string(),
        ],
    )
}

pub fn validate_command(
    response: &str,
    imei: &str,
    command_type: &str,
    content: &[&str],
) -> Result<(), &'static str> {
    let vendor = crate::config::VENDOR;

    let content_regex = content.join(",");

    let pattern = format!(
        r"^\*SCOR,{vendor},{imei},{command_type},{content}#\n",
        vendor = vendor,
        imei = imei,
        command_type = command_type,
        content = content_regex
    );
    let regex = Regex::new(&pattern).map_err(|_| "Failed to compile regex")?;

    if regex.is_match(response) {
        Ok(())
    } else {
        let error_message = format!(
            "Invalid response format: expected pattern '{}', got '{}'",
            pattern, response
        );
        Err(Box::leak(error_message.into_boxed_str()))
    }
}

// 1.3.8
// *SCOR,OM,123456789123456,S6,80,3,221,0,372,372,0,28#<LF>
// 1The current power of the scooter 80->80%
// 2Current mode of scooter 1:low speed
// 3Current speed 22->22km/h
// 4Scooter charging status 0->Not charged，1->Charging
// 5Battery 1 Voltage
// 6Battery 2 Voltage
// unit：0.1V
// 7Scooter status
// 0->unlocked，
// 8Current network signal value, the value from 2-32. The larger value the better signal.
// 9Single riding mileage unit: 10m 100->1000m
// unit：0.1V
// 2:medium speed
// 3:high speed
// 372->37.2V
// 372->37.2V（When the scooter has no a battery 2, it is 0 here.）
// 1->locked
// S7（Scooter setting instruction 1）
// Note: The following setting information is not saved after power-off, and the default value is restored after
// restarting or unlocking.
// <1><2><3><4>
// server->lock
// 1
// 2
// 3
// 4
// *SCOS,OM,123456789123456,S7,0,3,0,0#<LF>
// Headlight switch0:invalid（Don’t set） 1:shut down2:open
// （Defaults: 1:shut down）
// Mode setting0:invalid（Don’t set） 1:shut down2:Medium speed 3:high speed （Defaults:
// 2: shut down）
// Throttle response
// 0:invalid（Don’t set）1:shut down2:open（Defaults: 1:shut
// Taillights flashing 0:invalid（Don’t set）1:shut down2:open（Defaults: 1:shut
// down）
// down）
// <1><2><3><4>
// 8欧米智能
// lock->server*SCOR,OM,123456789123456,S7,0,3,0,0 #<LF>
// 1Headlight switch 0:invalid（Don’t set）
// 2Mode setting
// 3Throttle response 0:invalid（Don’t set）
// 4Taillights flashing 0:invalid（Don’t set） 1:shut down 2:open
// 1:shut down
// 0:invalid（Don’t set） 1:low speed
// 2:open
// 2:medium speed 3:high speed
// 1:shut down
// 2:open
