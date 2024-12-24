#[cfg(test)]
mod hearbeat_command_tests {
    use crate::commands::hearbeat_command::{ChargingStatus, ScooterStatus};

    // Tests for ScooterStatus
    #[test]
    fn test_scooter_status_with_valid_unlocked() {
        let status = ScooterStatus::try_from(0).unwrap();
        matches!(status, ScooterStatus::Unlocked);
    }

    #[test]
    fn test_scooter_status_with_valid_locked() {
        let status = ScooterStatus::try_from(1).unwrap();
        matches!(status, ScooterStatus::Locked);
    }

    #[test]
    fn test_scooter_status_with_invalid_value() {
        let result = ScooterStatus::try_from(2); // Invalid value
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid scooter status: 2");
    }

    // Tests for ChargingStatus
    #[test]
    fn test_charging_status_with_valid_uncharged() {
        let status = ChargingStatus::try_from(0).unwrap();
        matches!(status, ChargingStatus::Uncharged);
    }

    #[test]
    fn test_charging_status_with_valid_charging() {
        let status = ChargingStatus::try_from(1).unwrap();
        matches!(status, ChargingStatus::Charging);
    }

    #[test]
    fn test_charging_status_with_invalid_value() {
        let result = ChargingStatus::try_from(2); // Invalid value
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid charging status: 2");
    }
}
