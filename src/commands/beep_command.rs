#[derive(Debug)]
pub enum BeepPlayContent {
    Hold,
    FindScooterAlert,
    TurnOffVoice,
    TurnOnVoice,
}

impl TryFrom<u8> for BeepPlayContent {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BeepPlayContent::Hold),
            2 => Ok(BeepPlayContent::FindScooterAlert),
            80 => Ok(BeepPlayContent::TurnOffVoice),
            81 => Ok(BeepPlayContent::TurnOnVoice),
            _ => Err(format!("Invalid beep play content: {}", value)),
        }
    }
}
