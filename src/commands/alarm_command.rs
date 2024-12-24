#[derive(Debug)]
pub enum AlarmType {
    IllegalMovement,
    Falling,
    IllegalRemoval,
    LowPower,
    LiftedUp,
    IllegalDemolition,
}

impl TryFrom<u8> for AlarmType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AlarmType::IllegalMovement),
            2 => Ok(AlarmType::Falling),
            3 => Ok(AlarmType::IllegalRemoval),
            4 => Ok(AlarmType::LowPower),
            6 => Ok(AlarmType::LiftedUp),
            7 => Ok(AlarmType::IllegalDemolition),
            _ => Err(format!("Invalid alarm type: {}", value)),
        }
    }
}
