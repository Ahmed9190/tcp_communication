#[cfg(test)]
mod scooter_setting_command_tests {
    use crate::commands::scooter_setting_command::{
        HeadlightSwitch, ModeSetting, TaillightsFlashing, ThrottleResponse,
    };

    #[test]
    fn test_headlight_switch_with_valid_values() {
        let no_set = HeadlightSwitch::try_from(0).unwrap();
        matches!(no_set, HeadlightSwitch::NoSet);

        let shutdown = HeadlightSwitch::try_from(1).unwrap();
        matches!(shutdown, HeadlightSwitch::Shutdown);

        let open = HeadlightSwitch::try_from(2).unwrap();
        matches!(open, HeadlightSwitch::Open);
    }

    #[test]
    fn test_headlight_switch_with_invalid_value() {
        let result = HeadlightSwitch::try_from(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid headlight switch value: 3");
    }

    #[test]
    fn test_mode_setting_with_valid_values() {
        let no_set = ModeSetting::try_from(0).unwrap();
        matches!(no_set, ModeSetting::NoSet);

        let low_speed = ModeSetting::try_from(1).unwrap();
        matches!(low_speed, ModeSetting::LowSpeed);

        let medium_speed = ModeSetting::try_from(2).unwrap();
        matches!(medium_speed, ModeSetting::MediumSpeed);

        let high_speed = ModeSetting::try_from(3).unwrap();
        matches!(high_speed, ModeSetting::HighSpeed);
    }

    #[test]
    fn test_mode_setting_with_invalid_value() {
        let result = ModeSetting::try_from(4);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid mode setting value: 4");
    }

    #[test]
    fn test_throttle_response_with_valid_values() {
        let no_set = ThrottleResponse::try_from(0).unwrap();
        matches!(no_set, ThrottleResponse::NoSet);

        let shutdown = ThrottleResponse::try_from(1).unwrap();
        matches!(shutdown, ThrottleResponse::Shutdown);

        let open = ThrottleResponse::try_from(2).unwrap();
        matches!(open, ThrottleResponse::Open);
    }

    #[test]
    fn test_throttle_response_with_invalid_value() {
        let result = ThrottleResponse::try_from(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid throttle response value: 3");
    }

    #[test]
    fn test_taillights_flashing_with_valid_values() {
        let no_set = TaillightsFlashing::try_from(0).unwrap();
        matches!(no_set, TaillightsFlashing::NoSet);

        let shutdown = TaillightsFlashing::try_from(1).unwrap();
        matches!(shutdown, TaillightsFlashing::Shutdown);

        let open = TaillightsFlashing::try_from(2).unwrap();
        matches!(open, TaillightsFlashing::Open);
    }

    #[test]
    fn test_taillights_flashing_with_invalid_value() {
        let try_from = TaillightsFlashing::try_from(3);
        let result = try_from;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid taillights flashing value: 3");
    }
}
