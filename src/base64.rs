
/// Base64 编码表
const BASE64_CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 Encode
///
/// Example:
/// ```rust
/// use light_tool::base64;
/// println!("base64 encode: {}", base64::encode("Hello, World!"))
/// ```
pub fn encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in bytes {
        buffer = (buffer << 8) | (byte as u32);
        bits_collected += 8;

        while bits_collected >= 6 {
            bits_collected -= 6;
            let index = (buffer >> bits_collected) & 0b111111; // 取出高6位
            output.push(BASE64_CHARSET[index as usize]);
        }
    }

    // 如果有剩余位数，不足6位的部分补0
    if bits_collected > 0 {
        buffer <<= 6 - bits_collected; // 左移补零
        let index = buffer & 0b111111;
        output.push(BASE64_CHARSET[index as usize]);
    }

    // 添加必要的填充 '='
    while output.len() % 4 != 0 {
        output.push(b'=');
    }

    String::from_utf8(output).unwrap()
}

/// Base64 Decode
///
/// Example:
/// ```rust
/// use light_tool::base64;
/// println!("base64 decode: {}", base64::decode("SGVsbG8sIFdvcmxkIQ==").unwrap())
/// ```
pub fn decode(input: &str) -> Result<String, String> {
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in input.as_bytes() {
        if byte == b'=' {
            break; // 跳过填充字符
        }

        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return Err(format!("Invalid character in Base64 string: {}", byte)),
        };

        buffer = (buffer << 6) | (value as u32);
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            output.push((buffer >> bits_collected) as u8);
            buffer &= (1 << bits_collected) - 1; // 保留剩余位
        }
    }

    String::from_utf8(output).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("Hello, World!"), "SGVsbG8sIFdvcmxkIQ==");
        assert_eq!(encode("Rust is awesome!"), "UnVzdCBpcyBhd2Vzb21lIQ==");
        assert_eq!(encode(""), "");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("SGVsbG8sIFdvcmxkIQ==").unwrap(), "Hello, World!")
    }
}
