#[cfg(test)]
mod parse_command_tests {
    use crate::commands::parser::parse_command;

    #[test]
    fn test_parse_command_with_valid_data() {
        let raw_data = "*SCOR,LZ,123456789012345,R0,1,2,User1,Timestamp#\n";
        let result = parse_command(raw_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_command_with_invalid_header() {
        let raw_data = "*INVALID,LZ,123456789012345,R0,0,1,User1,Timestamp#\n";
        let result = parse_command(raw_data);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod parse_coordinates_tests {
    use crate::commands::parser::parse_coordinates;

    #[test]
    fn test_parse_coordinates_with_valid_latitude() {
        let result = parse_coordinates("2237.7514", "N");
        assert!((result.unwrap() - 22.62919).abs() < 1e-6);
    }

    #[test]
    fn test_parse_coordinates_with_valid_longitude() {
        let result = parse_coordinates("11408.6214", "E");
        assert!((result.unwrap() - 114.14369).abs() < 1e-6);
    }

    #[test]
    fn test_parse_coordinates_with_invalid_format() {
        let result = parse_coordinates("22.375514", "N");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_coordinates_with_missing_decimal_point() {
        let result = parse_coordinates("12345", "N"); // Invalid format
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_coordinates_with_invalid_hemisphere() {
        let result = parse_coordinates("2237.7514", "X");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_coordinates_with_southern_hemisphere() {
        let result = parse_coordinates("2237.7514", "S");
        assert!((result.unwrap() + 22.62919).abs() < 1e-6);
    }

    #[test]
    fn test_parse_coordinates_with_western_hemisphere() {
        let result = parse_coordinates("11408.6214", "W");
        assert!((result.unwrap() + 114.14369).abs() < 1e-6);
    }
}

#[cfg(test)]
mod parse_datetime_tests {
    use crate::commands::parser::parse_datetime;

    #[test]
    fn test_parse_datetime_with_valid_input() {
        let result = parse_datetime("123045", "151216").unwrap();
        assert_eq!(result.to_string(), "2016-12-15 12:30:45 UTC");
    }

    #[test]
    fn test_parse_datetime_with_invalid_time() {
        let result = parse_datetime("250045", "151216"); // Invalid time
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid time. Hours, minutes, or seconds out of range."
        );
    }

    #[test]
    fn test_parse_datetime_with_invalid_date() {
        let result = parse_datetime("123045", "321216"); // Invalid date
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid date. Day, month, or year out of range."
        );
    }
}

#[cfg(test)]
mod parse_time_tests {
    use crate::commands::parser::parse_time;

    #[test]
    fn test_parse_time_with_valid_input() {
        let result = parse_time("123045").unwrap();
        assert_eq!(result.to_string(), "12:30:45");
    }

    #[test]
    fn test_parse_time_with_short_input_length() {
        let result = parse_time("12304"); // Too short
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid time format. Expected hhmmss (6 characters)."
        );
    }

    #[test]
    fn test_parse_time_with_invalid_hours() {
        let result = parse_time("250045"); // Invalid hours
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid time. Hours, minutes, or seconds out of range."
        );
    }

    #[test]
    fn test_parse_time_with_invalid_minutes() {
        let result = parse_time("126045"); // Invalid minutes
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid time. Hours, minutes, or seconds out of range."
        );
    }

    #[test]
    fn test_parse_time_with_invalid_seconds() {
        let result = parse_time("123060"); // Invalid seconds
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid time. Hours, minutes, or seconds out of range."
        );
    }

    #[test]
    fn test_parse_time_with_non_numeric_characters() {
        let result = parse_time("12xx45"); // Non-numeric characters
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid minutes. Ensure it's a number."
        );
    }
}

#[cfg(test)]
mod parse_date_tests {
    use crate::commands::parser::parse_date;

    #[test]
    fn test_parse_date_with_valid_input() {
        let result = parse_date("151216").unwrap();
        assert_eq!(result.to_string(), "2016-12-15"); // 15th December 2016
    }

    #[test]
    fn test_parse_date_with_valid_leap_year() {
        let result = parse_date("290216").unwrap(); // Leap year
        assert_eq!(result.to_string(), "2016-02-29");
    }

    #[test]
    fn test_parse_date_with_short_input_length() {
        let result = parse_date("15121"); // Too short
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid date format. Expected ddmmyy (6 characters)."
        );
    }

    #[test]
    fn test_parse_date_with_invalid_day() {
        let result = parse_date("321216"); // Invalid day
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid date. Day, month, or year out of range."
        );
    }

    #[test]
    fn test_parse_date_with_invalid_month() {
        let result = parse_date("151316"); // Invalid month
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid date. Day, month, or year out of range."
        );
    }

    #[test]
    fn test_parse_date_with_non_numeric_characters() {
        let result = parse_date("abcd16"); // Non-numeric day and month
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid day. Ensure it's a number.");
    }
}
