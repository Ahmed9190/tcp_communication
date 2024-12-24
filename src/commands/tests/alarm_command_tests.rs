#[cfg(test)]
mod alarm_command_tests {
    use crate::commands::alarm_command::AlarmType;

    #[test]
    fn test_alarm_type_with_valid_illegal_movement() {
        let alarm = AlarmType::try_from(1).unwrap();
        matches!(alarm, AlarmType::IllegalMovement);
    }

    #[test]
    fn test_alarm_type_with_valid_falling() {
        let alarm = AlarmType::try_from(2).unwrap();
        matches!(alarm, AlarmType::Falling);
    }

    #[test]
    fn test_alarm_type_with_valid_illegal_removal() {
        let alarm = AlarmType::try_from(3).unwrap();
        matches!(alarm, AlarmType::IllegalRemoval);
    }

    #[test]
    fn test_alarm_type_with_valid_low_power() {
        let alarm = AlarmType::try_from(4).unwrap();
        matches!(alarm, AlarmType::LowPower);
    }

    #[test]
    fn test_alarm_type_with_valid_lifted_up() {
        let alarm = AlarmType::try_from(6).unwrap();
        matches!(alarm, AlarmType::LiftedUp);
    }

    #[test]
    fn test_alarm_type_with_valid_illegal_demolition() {
        let alarm = AlarmType::try_from(7).unwrap();
        matches!(alarm, AlarmType::IllegalDemolition);
    }

    #[test]
    fn test_alarm_type_with_invalid_value() {
        let result = AlarmType::try_from(0); // Invalid alarm type
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid alarm type: 0");
    }

    #[test]
    fn test_alarm_type_with_invalid_high_value() {
        let result = AlarmType::try_from(8); // Invalid alarm type
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid alarm type: 8");
    }
}
