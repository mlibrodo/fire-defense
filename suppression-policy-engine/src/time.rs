use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current UTC time in milliseconds since epoch.
pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
