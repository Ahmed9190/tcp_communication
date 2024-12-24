#[cfg(test)]
mod operation_tests {
    use crate::server::commands::R0Operation;

    #[test]
    fn test_operation_with_valid_unlock() {
        let operation = R0Operation::try_from("0");
        matches!(operation, Ok(R0Operation::Unlock));
    }

    #[test]
    fn test_operation_with_valid_lock() {
        let operation = R0Operation::try_from("1");
        matches!(operation, Ok(R0Operation::Lock));
    }

    #[test]
    fn test_operation_with_valid_rfid_card_unlock() {
        let operation = R0Operation::try_from("2");
        matches!(operation, Ok(R0Operation::RFIDCardUnlock));
    }

    #[test]
    fn test_operation_with_valid_rfid_card_lock() {
        let operation = R0Operation::try_from("3");
        matches!(operation, Ok(R0Operation::RFIDCardLock));
    }

    #[test]
    fn test_operation_with_invalid_value() {
        let result = R0Operation::try_from("4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid operation: 4");
    }
}

#[cfg(test)]
mod status_tests {
    use std::convert::TryFrom;

    use crate::commands::positioning_command::Status;

    #[test]
    fn test_status_with_valid_success() {
        let status = Status::try_from(0).unwrap();
        matches!(status, Status::Success);
    }

    #[test]
    fn test_status_with_valid_failure() {
        let status = Status::try_from(1).unwrap();
        matches!(status, Status::Failure);
    }

    #[test]
    fn test_status_with_valid_key_error() {
        let status = Status::try_from(2).unwrap();
        matches!(status, Status::KeyError);
    }

    #[test]
    fn test_status_with_invalid_value() {
        let result = Status::try_from(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid status code: 3");
    }
}

#[cfg(test)]
mod scooter_command_tests {
    use crate::{commands::scooter_command::ScooterCommand, server::commands::R0Operation};

    use std::convert::TryFrom;

    #[test]
    fn test_unlock_or_lock_response() {
        let parts: &[&str] = &[
            "*SCOR",
            "LZ",
            "123456789012345",
            "R0",
            "0",
            "255",
            "user1",
            "timestamp",
        ];
        let command = ScooterCommand::try_from(parts).unwrap();
        if let ScooterCommand::UnlockOrLockResponse {
            imei,
            operation,
            key,
            user_id,
            timestamp,
        } = command
        {
            assert_eq!(imei, "123456789012345");
            assert!(matches!(operation, R0Operation::Unlock));
            assert_eq!(key, 255);
            assert_eq!(user_id, "user1");
            assert_eq!(timestamp, "timestamp");
        } else {
            panic!("Parsed command is not UnlockOrLockResponse");
        }
    }

    #[test]
    fn test_invalid_command_format_with_insufficient_parts() {
        let parts: &[&str] = &["*SCOR", "LZ", "123456789012345", "Q0"];
        let result = ScooterCommand::try_from(parts);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid command format: insufficient parts"
        );
    }

    #[test]
    fn test_invalid_command() {
        let parts: &[&str] = &["*SCOR", "LZ", "123456789012345", "UNKNOWN", "0"];
        let result = ScooterCommand::try_from(parts);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown command: UNKNOWN");
    }
}
