use chrono::{Duration, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum UnlockStep {
    SendR0,
    WaitForR0Response,
    SendL0,
    WaitForL0Response,
    SendFinalL0,
    Completed,
}

#[derive(Debug)]
pub struct UnlockFlow {
    pub imei: String,
    pub correlation_id: String,
    pub current_step: UnlockStep,
    pub key_effective_time: u8,
    pub user_id: String,
    pub timestamp_1: String,
    pub timestamp_2: String,
}

impl UnlockFlow {
    pub fn new(imei: String, correlation_id: String, user_id: String) -> Self {
        let timestamp_1 = Utc::now().timestamp().to_string();
        let timestamp_2 = (Utc::now() + Duration::seconds(3)).timestamp().to_string();
        Self {
            imei,
            correlation_id,
            current_step: UnlockStep::SendR0,
            key_effective_time: 20,
            user_id,
            timestamp_1,
            timestamp_2,
        }
    }
}
