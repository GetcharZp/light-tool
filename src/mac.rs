use std::process::Command;
use crate::lazy::Lazy;
use crate::random;

pub static MAC: Lazy<String> = Lazy::new(|| {
    address().unwrap_or(random::alpha(17))
});

/// Returns one MAC address of the current machine
///
/// # Example
///
/// ```no_run
/// use light_tool::mac;
/// println!("mac address: {}", mac::address().unwrap())
/// ```
pub fn address() -> Option<String> {
    let output = match std::env::consts::OS {
        "linux" => Command::new("ifconfig").output(),
        "macos" => Command::new("ifconfig").output(),
        "windows" => Command::new("getmac").output(),
        _ => return None,
    };

    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                for mac_candidate in line.split_whitespace() {
                    if is_valid_address(mac_candidate) {
                        return Some(mac_candidate.to_string());
                    }
                }
            }
        }
    }
    None
}

fn is_valid_address(mac: &str) -> bool {
    // windows mac address -> C4-75-AB-75-9C-25
    // linux mac address -> 5A:9A:C3:47:2D:33
    let parts: Vec<&str> = mac.split(|c| c == ':' || c == '-').collect();
    parts.len() == 6 && parts.iter().all(|p| p.len() == 2 && p.chars().all(|c| c.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mac_address() {
        println!("mac address: {:?}", address());
    }
}
