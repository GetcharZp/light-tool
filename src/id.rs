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
}