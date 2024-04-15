use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_as_millis() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since_epoch.as_millis()
}