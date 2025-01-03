#[derive(Debug)]
pub enum Turn {
    Off,
    On,
    DontSet,
}

impl From<&Turn> for u8 {
    fn from(turn: &Turn) -> u8 {
        match turn {
            Turn::DontSet => 0,
            Turn::Off => 1,
            Turn::On => 2,
        }
    }
}

#[derive(Debug)]
pub enum SpeedMode {
    Low,
    Medium,
    High,
    DontSet,
}
impl From<&SpeedMode> for u8 {
    fn from(speed: &SpeedMode) -> u8 {
        match speed {
            SpeedMode::DontSet => 0,
            SpeedMode::Low => 1,
            SpeedMode::Medium => 2,
            SpeedMode::High => 3,
        }
    }
}
impl TryFrom<u8> for SpeedMode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SpeedMode::DontSet),
            1 => Ok(SpeedMode::Low),
            2 => Ok(SpeedMode::Medium),
            3 => Ok(SpeedMode::High),
            _ => Err("Out of range 0, 1, 2, 3".to_string()),
        }
    }
}
