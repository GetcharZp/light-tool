use crate::{random, timestamp, mac, md5};

/// Generate uuid by random_str - timestamp - mac
///
/// # Example
///
/// ```no_run
/// use light_tool::uuid;
/// println!("uuid: {}", uuid::new())
/// ```
pub fn new() -> String {
    let s =md5::str(format!("{}-{}-{}",  random::alpha_num(36),
                            timestamp::nano_seconds(),
                            mac::address().unwrap_or(timestamp::nano_seconds().to_string())));

    format!("{}-{}-{}-{}-{}", &s[0..8], &s[8..12], &s[12..16], &s[16..20], &s[20..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        println!("uuid: {}", new());
    }
}