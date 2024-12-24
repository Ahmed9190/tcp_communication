#[cfg(test)]
mod validate_l0_response_tests {
    use crate::server::{commands::R0Operation, protocol::validate_l0_response};

    #[test]
    fn test_validate_l0_response_valid() {
        let imei = "123456789123456";
        let user_id = 1234u32;
        let timestamp = 1497689816;
        let response = format!(
            "*SCOR,{vendor},{imei},L0,0,{user_id},{timestamp}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            user_id = user_id,
            timestamp = timestamp
        );

        let result = validate_l0_response(&response, imei, user_id, timestamp);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_l0_response_invalid_content() {
        let imei = "123456789123456";
        let invalid_operation: &str = (&R0Operation::Lock).try_into().unwrap();
        let user_id = 1234u32;
        let timestamp = 1497689816;
        let response = format!(
            "*SCOR,{vendor},{imei},L0,{invalid_operation},{user_id},{timestamp}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            invalid_operation = invalid_operation,
            user_id = user_id,
            timestamp = timestamp
        );

        let result = validate_l0_response(&response, imei, user_id, timestamp);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod validate_r0_response_tests {
    use crate::server::{commands::R0Operation, protocol::validate_r0_response};

    #[test]
    fn test_validate_r0_response_unlock_valid() {
        let imei = "123456789123456";
        let r0_operation = R0Operation::Unlock;
        let operation: &str = (&r0_operation).try_into().unwrap();
        let user_id = 1234u32;
        let timestamp = 1497689816;
        let response = format!(
            "*SCOR,{vendor},{imei},R0,{operation},55,{user_id},{timestamp}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            operation = operation,
            user_id = user_id,
            timestamp = timestamp
        );

        let result = validate_r0_response(&response, imei, &r0_operation, user_id, timestamp);

        assert_eq!(result.unwrap(), "55");
    }

    #[test]
    fn test_validate_r0_response_lock_valid() {
        let imei = "123456789123456";
        let r0_operation = R0Operation::Lock;
        let operation: &str = (&r0_operation).try_into().unwrap();
        let user_id = 1234u32;
        let timestamp = 1497689816;
        let response = format!(
            "*SCOR,{vendor},{imei},R0,{operation},55,{user_id},{timestamp}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            operation = operation,
            user_id = user_id,
            timestamp = timestamp
        );

        let result = validate_r0_response(&response, imei, &r0_operation, user_id, timestamp);

        assert_eq!(result.unwrap(), "55");
    }

    #[test]
    fn test_validate_r0_response_invalid_format() {
        let imei = "123456789123456";

        let r0_operation = R0Operation::Lock;
        let user_id = 1234u32;
        let timestamp = 1497689816;
        let response = "*SCOR,INVALID,123456789123456,R0,0,55,1234,1497689816#\n";

        let result = validate_r0_response(&response, imei, &r0_operation, user_id, timestamp);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod validate_l1_response_tests {
    use crate::server::protocol::validate_l1_response;

    #[test]
    fn test_validate_l1_response_valid() {
        let imei = "123456789123456";
        let user_id = 1234u32;
        let timestamp = rand::random::<u32>();
        let cycling_time = rand::random::<u8>();
        let response = format!(
            "*SCOR,{vendor},{imei},L1,0,{user_id},{timestamp},{cycling_time}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            user_id = user_id,
            timestamp = timestamp,
            cycling_time = cycling_time
        );

        let result = validate_l1_response(&response, imei, user_id);

        assert!(result.is_ok());
    }
}
#[cfg(test)]
mod validate_response_tests {
    use crate::server::protocol::validate_command;

    #[test]
    fn test_validate_command_valid() {
        let imei = "123456789123456";
        let command_type = "CUSTOM";
        let content = &["content1", "content2"];
        let response = format!(
            "*SCOR,{vendor},{imei},{command_type},{content1},{content2}#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            command_type = command_type,
            content1 = content[0],
            content2 = content[1]
        );

        let result = validate_command(&response, imei, command_type, content);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_command_invalid_vendor() {
        let imei = "123456789123456";
        let command_type = "CUSTOM";
        let content = &["content1", "content2"];
        let response = format!(
            "*SCOR,INVALID,{imei},{command_type},{content1},{content2}#\n",
            imei = imei,
            command_type = command_type,
            content1 = content[0],
            content2 = content[1]
        );

        let result = validate_command(&response, imei, command_type, content);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_command_invalid_content() {
        let imei = "123456789123456";
        let command_type = "CUSTOM";
        let content = &["content1", "content2"];
        let response = format!(
            "*SCOR,{vendor},{imei},{command_type},content3,content4#\n",
            vendor = crate::config::VENDOR,
            imei = imei,
            command_type = command_type
        );

        let result = validate_command(&response, imei, command_type, content);

        assert!(result.is_err());
    }
}
