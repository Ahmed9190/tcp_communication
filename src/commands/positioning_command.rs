use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum PositioningIdentifier {
    ObtainPositioning,
    PositionTracking,
}

impl TryFrom<u8> for PositioningIdentifier {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PositioningIdentifier::ObtainPositioning),
            1 => Ok(PositioningIdentifier::PositionTracking),
            _ => Err(format!("Invalid positioning identifier: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum PositioningStatus {
    Effective,
    Invalid,
}

impl TryFrom<char> for PositioningStatus {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(PositioningStatus::Effective),
            'V' => Ok(PositioningStatus::Invalid),
            _ => Err(format!("Invalid positioning status: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum Hemisphere {
    North,
    South,
    East,
    West,
}

impl TryFrom<&str> for Hemisphere {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Hemisphere::North),
            "S" => Ok(Hemisphere::South),
            "E" => Ok(Hemisphere::East),
            "W" => Ok(Hemisphere::West),
            _ => Err(format!("Invalid hemisphere: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Autonomous,
    Differential,
    Estimate,
    InvalidData,
}

impl TryFrom<char> for Mode {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Mode::Autonomous),
            'D' => Ok(Mode::Differential),
            'E' => Ok(Mode::Estimate),
            'N' => Ok(Mode::InvalidData),
            _ => Err(format!("Invalid mode: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct PositioningResponse {
    pub imei: String,
    pub identifier: PositioningIdentifier,
    pub utc_datetime: DateTime<Utc>,
    pub positioning_status: PositioningStatus,
    pub latitude: f64,
    pub latitude_hemisphere: Hemisphere,
    pub longitude: f64,
    pub longitude_hemisphere: Hemisphere,
    pub satellites_number: u8,
    pub positioning_accuracy: f32,
    pub altitude: f32,
    pub mode: Mode,
}

impl PositioningResponse {
    /// Converts latitude to WGS84 format.
    pub fn convert_latitude_to_wgs84(&self) -> f64 {
        let degrees = (self.latitude / 100.0).floor();
        let minutes = self.latitude % 100.0;
        let converted = degrees + (minutes / 60.0);
        match self.latitude_hemisphere {
            Hemisphere::North => converted,  // North is positive
            Hemisphere::South => -converted, // South is negative
            _ => panic!(
                "Invalid hemisphere for latitude: {:?}",
                self.latitude_hemisphere,
            ),
        }
    }

    /// Converts longitude to WGS84 format.
    pub fn convert_longitude_to_wgs84(&self) -> f64 {
        let degrees = (self.longitude / 100.0).floor();
        let minutes = self.longitude % 100.0;
        let converted = degrees + (minutes / 60.0);
        match self.longitude_hemisphere {
            Hemisphere::East => converted,  // East is positive
            Hemisphere::West => -converted, // West is negative
            _ => panic!(
                "Invalid hemisphere for longitude: {:?}",
                self.longitude_hemisphere,
            ),
        }
    }

    /// Provides a summary of the positioning in WGS84 format.
    pub fn positioning_summary(&self) -> String {
        let wgs84_lat = self.convert_latitude_to_wgs84();
        let wgs84_lon = self.convert_longitude_to_wgs84();

        format!(
            "IMEI: {}, WGS84 Coordinates: (Lat: {}, Lon: {}), Altitude: {}m, Status: {:?}, Mode: {:?}",
            self.imei, wgs84_lat, wgs84_lon, self.altitude, self.positioning_status, self.mode
        )
    }
}

#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    KeyError,
}

impl TryFrom<u8> for Status {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::KeyError),
            _ => Err(format!("Invalid status code: {}", value)),
        }
    }
}
