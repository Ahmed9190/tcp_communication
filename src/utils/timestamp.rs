use chrono::Utc;

pub fn current() -> i64 {
    Utc::now().timestamp()
}
