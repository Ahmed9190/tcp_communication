#[derive(Debug)]
pub enum ScooterStatus {
    Unlocked,
    Locked,
}

impl TryFrom<u8> for ScooterStatus {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScooterStatus::Unlocked),
            1 => Ok(ScooterStatus::Locked),
            _ => Err(format!("Invalid scooter status: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ChargingStatus {
    Uncharged,
    Charging,
}

impl TryFrom<u8> for ChargingStatus {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChargingStatus::Uncharged),
            1 => Ok(ChargingStatus::Charging),
            _ => Err(format!("Invalid charging status: {}", value)),
        }
    }
}
