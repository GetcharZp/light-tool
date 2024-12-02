use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in seconds
///
/// # Example
///
/// ```no_run
/// use light_tool::timestamp;
/// println!("second timestamp: {}", timestamp::seconds());
/// ```
pub fn seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Returns the current time in milliseconds
///
/// # Example
///
/// ```no_run
/// use light_tool::timestamp;
/// println!("milli second timestamp: {}", timestamp::milli_seconds());
/// ```
pub fn milli_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Returns the current time in nanoseconds
///
/// # Example
///
/// ```no_run
/// use light_tool::timestamp;
/// println!("nano second timestamp: {}", timestamp::nano_seconds());
/// ```
pub fn nano_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_nanos() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds() {
        let seconds = seconds();
        println!("seconds: {}", seconds);
        assert!(seconds > 0);
    }

    #[test]
    fn test_milli_seconds() {
        let milli_seconds = milli_seconds();
        println!("milli_seconds: {}", milli_seconds);
        assert!(milli_seconds > 0);
    }

    #[test]
    fn test_nano_seconds() {
        let nano_seconds = nano_seconds();
        println!("nano_seconds: {}", nano_seconds);
        assert!(nano_seconds > 0);
    }
}
