use chrono::Utc;

pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}
