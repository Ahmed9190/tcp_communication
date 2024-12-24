#[cfg(test)]
mod beep_command_tests {
    use crate::commands::beep_command::BeepPlayContent;

    #[test]
    fn test_beep_play_content_with_valid_hold() {
        let beep = BeepPlayContent::try_from(1).unwrap();
        matches!(beep, BeepPlayContent::Hold);
    }

    #[test]
    fn test_beep_play_content_with_valid_find_scooter_alert() {
        let beep = BeepPlayContent::try_from(2).unwrap();
        matches!(beep, BeepPlayContent::FindScooterAlert);
    }

    #[test]
    fn test_beep_play_content_with_valid_turn_off_voice() {
        let beep = BeepPlayContent::try_from(80).unwrap();
        matches!(beep, BeepPlayContent::TurnOffVoice);
    }

    #[test]
    fn test_beep_play_content_with_valid_turn_on_voice() {
        let beep = BeepPlayContent::try_from(81).unwrap();
        matches!(beep, BeepPlayContent::TurnOnVoice);
    }

    #[test]
    fn test_beep_play_content_with_invalid_low_value() {
        let result = BeepPlayContent::try_from(0); // Invalid value
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid beep play content: 0");
    }

    #[test]
    fn test_beep_play_content_with_invalid_high_value() {
        let result = BeepPlayContent::try_from(100); // Invalid value
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid beep play content: 100");
    }
}
