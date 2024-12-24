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
