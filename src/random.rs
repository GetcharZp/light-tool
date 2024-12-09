use crate::timestamp;

/// 基于异或位移实现
struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| timestamp::nano_seconds());
        XorShiftRng { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state >> 17;
        self.state
    }

    fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        min + self.next_u64() % (max - min)
    }

    fn gen_letter(&mut self, letters :&str) -> char {
        let idx = self.gen_range(0, letters.len() as u64) as usize;
        letters.as_bytes()[idx] as char
    }

    fn gen_random(&mut self, s :&str, length: usize) -> String {
        (0..length).map(|_| self.gen_letter(s)).collect()
    }
}

/// Generate random string
///
/// # Example
///
/// ```no_run
/// use light_tool::random;
/// println!("random string: {}", random::str("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 10))
/// ```
pub fn str(s :&str, len: usize) -> String {
    XorShiftRng::new(None).gen_random(s, len)
}

/// Generate random number
///
/// # Example
///
/// ```no_run
/// use light_tool::random;
/// println!("random number: {}", random::num(6))
/// ```
pub fn num(len: usize) -> String {
    str("0123456789", len)
}

/// Generate random alpha
///
/// # Example
/// ```no_run
/// use light_tool::random;
/// println!("random alpha: {}", random::alpha(6))
/// ```
pub fn alpha(len: usize) -> String {
    str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", len)
}

/// Generate random alpha number
///
/// # Example
/// ```no_run
/// use light_tool::random;
/// println!("random alpha number: {}", random::alpha_num(6))
/// ```
pub fn alpha_num(len: usize) -> String {
    str("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", len)
}

/// Generate random number in range
///
/// # Example
/// ```no_run
/// use light_tool::random;
/// println!("random range: {}", random::range(1, 10))
/// ```
pub fn range(min: u64, max: u64) -> u64 {
    XorShiftRng::new(None).gen_range(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_str() {
        println!("random_str: {}", str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 10));
    }

    #[test]
    fn test_random_num() {
        println!("random_num: {}", num(6));
    }

    #[test]
    fn test_random_alpha() {
        println!("random_alpha: {}", alpha(8));
    }

    #[test]
    fn test_random_alpha_num() {
        println!("random_alpha_num: {}", alpha_num(16));
    }

    #[test]
    fn test_random_range() {
        println!("random_range: {}", range(1, 3));
    }
}
