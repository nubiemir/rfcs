use std::time::{SystemTime, UNIX_EPOCH};

pub mod rfc862;
pub mod rfc865;
pub mod rfc867;

pub fn random(m: u64) -> u64 {
    let num = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % m;

    num
}
