use std::process;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::{random, timestamp, mac, md5};
use crate::lazy::Lazy;
use std::sync::Mutex;


/// Generate uuid by random_str - timestamp - mac
///
/// # Example
///
/// ```rust
/// use light_tool::id;
/// println!("uuid: {}", id::uuid())
/// ```
pub fn uuid() -> String {
    let s =md5::str(format!("{}-{}-{}",  random::alpha_num(36),
                            timestamp::nano_seconds(),
                            mac::address().unwrap_or(timestamp::nano_seconds().to_string())));

    format!("{}-{}-{}-{}-{}", &s[0..8], &s[8..12], &s[12..16], &s[16..20], &s[20..])
}

// 雪花ID生成器的配置常量
const EPOCH: u64 = 1545667200000; // 自定义起始时间戳（2018-18-25 00:00:00 UTC）
const MACHINE_ID: u64 = 0;         // 单节点固定机器ID
const MACHINE_ID_BITS: u64 = 5;    // 机器ID占用的位数
const SEQUENCE_BITS: u64 = 12;     // 序列号占用的位数
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1; // 序列号的最大值（4095）

// 雪花ID生成器结构体
struct SnowflakeIdGenerator {
    last_timestamp: u64, // 上次生成ID的时间戳
    sequence: u64,       // 当前毫秒内的序列号
}

impl SnowflakeIdGenerator {
    // 创建一个新的生成器实例
    fn new() -> Self {
        Self {
            last_timestamp: 0,
            sequence: 0,
        }
    }

    // 核心方法：生成一个唯一的雪花ID
    fn generate_id(&mut self) -> u64 {
        let mut current_timestamp = timestamp::milli_seconds();

        if current_timestamp < self.last_timestamp {
            // 系统时间倒退的情况处理：补偿时间（将时间设置为上次时间戳）
            current_timestamp = self.last_timestamp;
        }

        if current_timestamp == self.last_timestamp {
            // 同一毫秒内，增加序列号
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;
            if self.sequence == 0 {
                // 如果序列号达到最大值，等待下一毫秒
                current_timestamp = Self::wait_for_next_millis(current_timestamp);
            }
        } else {
            // 不同毫秒，重置序列号
            self.sequence = 0;
        }

        self.last_timestamp = current_timestamp; // 更新最后时间戳

        // 使用位移运算生成ID
        ((current_timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS))
            | (MACHINE_ID << SEQUENCE_BITS)
            | self.sequence
    }

    // 等待直到下一毫秒（用于解决序列号溢出）
    fn wait_for_next_millis(last_timestamp: u64) -> u64 {
        let mut timestamp = timestamp::milli_seconds();
        while timestamp <= last_timestamp {
            timestamp = timestamp::milli_seconds();
        }
        timestamp
    }
}

// 静态变量使用自定义的 OnceCell 进行延迟初始化
static GENERATOR: Lazy<Mutex<SnowflakeIdGenerator>> = Lazy::new(|| Mutex::new(SnowflakeIdGenerator::new()));

/// Generate snowflake ID (雪花码)
///
/// # Example
///
/// ```rust
/// use light_tool::id;
/// println!("snowflake id: {}", id::snowflake_id())
/// ```
pub fn snowflake_id() -> u64 {
    let mut generator = GENERATOR.get().lock().unwrap();
    generator.generate_id()
}

/// A struct representing a MongoDB ObjectId
struct ObjectId {
    bytes: [u8; 12],
}

// Global counter for uniqueness
static COUNTER: AtomicU32 = AtomicU32::new(0);

impl ObjectId {
    /// Generate a new ObjectId
    pub fn new() -> Self {
        let mut bytes = [0u8; 12];

        // 1. Add the current timestamp (4 bytes)
        bytes[..4].copy_from_slice(&(timestamp::seconds() as u32).to_be_bytes());

        // 2. Add the machine identifier (5 bytes)
        let mac = mac::address().unwrap_or(random::alpha(16));
        bytes[4..9].copy_from_slice(&mac.as_bytes()[..5]);

        // 3. Add the process ID (2 bytes)
        let pid = process::id() as u16;
        bytes[9..11].copy_from_slice(&pid.to_be_bytes());

        // 4. Add the counter (3 bytes)
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst) & 0xFFFFFF; // Keep only 3 bytes
        bytes[11] = (counter & 0xFF) as u8;         // Lowest byte
        bytes[10] = ((counter >> 8) & 0xFF) as u8;  // Middle byte
        bytes[9] |= ((counter >> 16) & 0xFF) as u8; // Highest byte (shared with PID)

        ObjectId { bytes }
    }

    /// Convert the ObjectId to a hexadecimal string
    pub fn to_hex(&self) -> String {
        self.bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

/// Generate MongoDB ObjectID
///
/// # Example
///
/// ```rust
/// use light_tool::id;
/// println!("object id: {}", id::object_id())
/// ```
pub fn object_id() -> String {
    ObjectId::new().to_hex()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        println!("uuid: {}", uuid());
    }

    #[test]
    fn test_snowflake_id() {
        for _ in 0..10 {
            println!("snowflake id: {}", snowflake_id());
        }
    }

    #[test]
    fn test_object_id() {
        for _ in 0..16 {
            println!("ObjectId: {}", object_id());
        }
    }
}