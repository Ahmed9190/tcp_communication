use super::scooter_command::ScooterCommand;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use regex::Regex;

#[derive(Debug)]
pub struct ParsedCommand {
    pub imei: String,
    pub command: String,
    pub additional_fields: Vec<String>,
}

pub fn parse_command(raw_data: &str) -> Result<ScooterCommand, String> {
    let raw_data = raw_data.trim_end_matches("#\n");
    let parts: Vec<&str> = raw_data.split(',').collect();

    // Validate header and vendor code
    if parts.get(0) != Some(&"*SCOR") {
        return Err(format!("Invalid header: {}", parts.get(0).unwrap_or(&"")));
    }
    if parts.get(1) != Some(&"LZ") {
        return Err(format!(
            "Unsupported vendor code: {}",
            parts.get(1).unwrap_or(&"")
        ));
    }

    ScooterCommand::try_from(&parts[..])
}

pub fn parse_coordinates(value: &str, hemisphere: &str) -> Result<f64, String> {
    // Define regex for latitude and longitude formats
    let lat_regex = Regex::new(r"^\d{2}\d{2}\.\d{4}$").unwrap(); // ddmm.mmmm
    let lng_regex = Regex::new(r"^\d{3}\d{2}\.\d{4}$").unwrap(); // dddmm.mmmm

    // Validate format based on expected input
    if !lat_regex.is_match(value) && !lng_regex.is_match(value) {
        return Err("Invalid coordinate format. Expected ddmm.mmmm or dddmm.mmmm.".to_string());
    }

    // Ensure the input is a valid number
    let value = value
        .trim()
        .parse::<f64>()
        .map_err(|_| "Invalid coordinate format. Ensure it is a number.")?;

    // Split the value into degrees and minutes
    let degrees = (value as i64 / 100) as f64; // Extract the degrees (integer part divided by 100)
    let minutes = value % 100.0; // Extract the minutes (remainder of the division)

    // Ensure minutes are within the valid range
    if !(0.0..60.0).contains(&minutes) {
        return Err("Minutes should be in the range 0 to 59.999".to_string());
    }

    // Calculate the WGS84 coordinate
    let coordinate = degrees + (minutes / 60.0);

    // Adjust for hemisphere
    match hemisphere.trim() {
        "N" | "E" => Ok(coordinate),
        "S" | "W" => Ok(-coordinate),
        _ => Err("Invalid hemisphere. Expected one of: N, S, E, W.".to_string()),
    }
}

pub fn parse_datetime(hhmmss: &str, ddmmyy: &str) -> Result<DateTime<Utc>, String> {
    // Parse time (hhmmss)
    let time = parse_time(hhmmss)?;

    // Parse date (ddmmyy)
    let date = parse_date(ddmmyy)?;

    // Combine the date and time into a DateTime
    Ok(Utc.from_utc_datetime(&NaiveDateTime::new(date, time)))
}

pub fn parse_time(hhmmss: &str) -> Result<NaiveTime, String> {
    // Validate input length
    if hhmmss.len() != 6 {
        return Err("Invalid time format. Expected hhmmss (6 characters).".to_string());
    }

    // Parse hours, minutes, and seconds
    let hours = hhmmss[0..2]
        .parse::<u32>()
        .map_err(|_| "Invalid hours. Ensure it's a number.".to_string())?;
    let minutes = hhmmss[2..4]
        .parse::<u32>()
        .map_err(|_| "Invalid minutes. Ensure it's a number.".to_string())?;
    let seconds = hhmmss[4..6]
        .parse::<u32>()
        .map_err(|_| "Invalid seconds. Ensure it's a number.".to_string())?;

    // Construct the NaiveTime, checking for out-of-range values
    NaiveTime::from_hms_opt(hours, minutes, seconds)
        .ok_or_else(|| "Invalid time. Hours, minutes, or seconds out of range.".to_string())
}

pub fn parse_date(ddmmyy: &str) -> Result<NaiveDate, String> {
    // Ensure the input has exactly 6 characters
    if ddmmyy.len() != 6 {
        return Err("Invalid date format. Expected ddmmyy (6 characters).".to_string());
    }

    // Parse day, month, and year
    let day = ddmmyy[0..2]
        .parse::<u32>()
        .map_err(|_| "Invalid day. Ensure it's a number.".to_string())?;
    let month = ddmmyy[2..4]
        .parse::<u32>()
        .map_err(|_| "Invalid month. Ensure it's a number.".to_string())?;
    let year = 2000
        + ddmmyy[4..6]
            .parse::<i32>()
            .map_err(|_| "Invalid year. Ensure it's a number.".to_string())?; // Assumes 21st century

    // Construct the NaiveDate, checking for out-of-range values
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| "Invalid date. Day, month, or year out of range.".to_string())
}
