#[cfg(test)]
mod positioning_identifier_tests {
    use crate::commands::positioning_command::PositioningIdentifier;
    use std::convert::TryFrom;

    #[test]
    fn test_positioning_identifier_with_valid_obtain_positioning() {
        let identifier = PositioningIdentifier::try_from(0).unwrap();
        matches!(identifier, PositioningIdentifier::ObtainPositioning);
    }

    #[test]
    fn test_positioning_identifier_with_valid_position_tracking() {
        let identifier = PositioningIdentifier::try_from(1).unwrap();
        matches!(identifier, PositioningIdentifier::PositionTracking);
    }

    #[test]
    fn test_positioning_identifier_with_invalid_value() {
        let result = PositioningIdentifier::try_from(2); // Invalid value
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid positioning identifier: 2");
    }
}
