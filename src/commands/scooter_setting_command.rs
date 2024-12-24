#[derive(Debug)]
pub enum HeadlightSwitch {
    NoSet,
    Shutdown,
    Open,
}

impl TryFrom<u8> for HeadlightSwitch {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HeadlightSwitch::NoSet),
            1 => Ok(HeadlightSwitch::Shutdown),
            2 => Ok(HeadlightSwitch::Open),
            _ => Err(format!("Invalid headlight switch value: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ModeSetting {
    NoSet,
    LowSpeed,
    MediumSpeed,
    HighSpeed,
}

impl TryFrom<u8> for ModeSetting {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ModeSetting::NoSet),
            1 => Ok(ModeSetting::LowSpeed),
            2 => Ok(ModeSetting::MediumSpeed),
            3 => Ok(ModeSetting::HighSpeed),
            _ => Err(format!("Invalid mode setting value: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ThrottleResponse {
    NoSet,
    Shutdown,
    Open,
}

impl TryFrom<u8> for ThrottleResponse {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ThrottleResponse::NoSet),
            1 => Ok(ThrottleResponse::Shutdown),
            2 => Ok(ThrottleResponse::Open),
            _ => Err(format!("Invalid throttle response value: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum TaillightsFlashing {
    NoSet,
    Shutdown,
    Open,
}

impl TryFrom<u8> for TaillightsFlashing {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TaillightsFlashing::NoSet),
            1 => Ok(TaillightsFlashing::Shutdown),
            2 => Ok(TaillightsFlashing::Open),
            _ => Err(format!("Invalid taillights flashing value: {}", value)),
        }
    }
}
