#[cfg(test)]
mod generate_r0_command_tests {
    use crate::server::commands::{self, R0Operation};

    #[test]
    fn test_generate_r0_command_unlock() {
        let imei = "123456789123456";
        let r0_operation = R0Operation::Unlock;
        let key_duration = 20;
        let user_id = 1234u32;
        let timestamp = 1497689816;

        let result =
            commands::generate_r0_command(imei, &r0_operation, key_duration, user_id, timestamp);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,R0,0,20,1234,1497689816#\n",
                vendor = crate::config::VENDOR
            )
        );
    }

    #[test]
    fn test_generate_r0_command_lock() {
        let imei = "123456789123456";
        let r0_operation = R0Operation::Lock;
        let key_duration = 20;
        let user_id = 1234;
        let timestamp = 1497689816;

        let result =
            commands::generate_r0_command(imei, &r0_operation, key_duration, user_id, timestamp);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,R0,1,20,1234,1497689816#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}

#[cfg(test)]
mod generate_l0_command_tests {
    use crate::server::commands;

    #[test]
    fn test_generate_l0_command_unlock() {
        let imei = "123456789123456";
        let key = "55";
        let user_id = 1234u32;
        let timestamp = 1497689816;

        let result = commands::generate_l0_command(imei, key, user_id, timestamp);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L0,55,1234,1497689816#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}

#[cfg(test)]
mod generate_final_ack_tests {
    use crate::server::commands;

    #[test]
    fn test_generate_final_ack() {
        let imei = "123456789123456";

        let result = commands::generate_l0_ack(imei);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L0#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}

#[cfg(test)]
mod generate_s7_command_tests {
    use crate::server::command_enums::{SpeedMode, Turn};
    use crate::server::commands;

    #[test]
    fn test_generate_s7_command() {
        let imei = "123456789123456";
        let headlight = Turn::On;
        let speed_mode = SpeedMode::Medium;
        let throttle = Turn::Off;
        let taillights_flashing = Turn::DontSet;

        let result = commands::generate_s7_command(
            imei,
            &headlight,
            &speed_mode,
            &throttle,
            &taillights_flashing,
        );

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,S7,2,2,1,0#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}
#[cfg(test)]
mod generate_command_tests {
    use crate::server::commands;

    #[test]
    fn test_generate_command_with_simple_content() {
        let imei = "123456789123456";
        let command = "R0";
        let content = &["0", "20", "1234", "1497689816"];

        let result = commands::generate_command(imei, command, content);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,R0,0,20,1234,1497689816#\n",
                vendor = crate::config::VENDOR
            )
        );
    }

    #[test]
    fn test_generate_command_with_multiple_content_items() {
        let imei = "123456789123456";
        let command = "L0";
        let content = &["55", "1234", "1497689816"];

        let result = commands::generate_command(imei, command, content);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L0,55,1234,1497689816#\n",
                vendor = crate::config::VENDOR
            )
        );
    }

    #[test]
    fn test_generate_command_empty_content() {
        let imei = "123456789123456";
        let command = "L0";
        let content: &[&str] = &[];

        let result = commands::generate_command(imei, command, content);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L0#\n",
                vendor = crate::config::VENDOR
            )
        );
    }

    #[test]
    fn test_generate_command_with_special_characters_in_content() {
        let imei = "123456789123456";
        let command = "CUSTOM";
        let content = &["special-content", "1234", "with,comma"];

        let result = commands::generate_command(imei, command, content);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,CUSTOM,special-content,1234,with,comma#\n",
                vendor = crate::config::VENDOR
            )
        );
    }

    #[test]
    fn test_generate_command_with_large_content() {
        let imei = "123456789123456";
        let command = "DATA";
        let content = &[
            "1234567890",
            "ABCDEFGHIJ",
            "special&characters",
            "with=equals",
            "spaces allowed",
        ];

        let result = commands::generate_command(imei, command, content);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,DATA,1234567890,ABCDEFGHIJ,special&characters,with=equals,spaces allowed#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}

#[cfg(test)]
mod generate_l1_command_tests {
    use crate::server::commands;

    #[test]
    fn test_generate_l1_command() {
        let imei = "123456789123456";
        let key = "55";

        let result = commands::generate_l1_command(imei, key);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L1,55#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}

#[cfg(test)]
mod generate_l1_ack_tests {
    use crate::server::commands;

    #[test]
    fn test_generate_l1_ack() {
        let imei = "123456789123456";

        let result = commands::generate_l1_ack(imei);

        assert_eq!(
            result,
            format!(
                "0xFFFF*SCOS,{vendor},123456789123456,L1#\n",
                vendor = crate::config::VENDOR
            )
        );
    }
}
