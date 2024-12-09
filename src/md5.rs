/// MD5 结构体
pub struct Md5 {
    data: Vec<u8>,       // 原始数据块
    bit_len: u64,        // 消息总位数
    state: [u32; 4],     // MD5 状态变量（A, B, C, D）
}

impl Md5 {
    /// 初始化 MD5 状态变量
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            bit_len: 0,
            state: [
                0x67452301, // A
                0xefcdab89, // B
                0x98badcfe, // C
                0x10325476, // D
            ],
        }
    }

    /// 更新 MD5 数据块
    pub fn update(&mut self, input: &[u8]) {
        self.data.extend_from_slice(input);
        self.bit_len += (input.len() as u64) * 8;
        while self.data.len() >= 64 {
            let block: [u8; 64] = self.data[..64].try_into().unwrap();
            self.process_block(&block);
            self.data.drain(..64);
        }
    }

    /// 处理 64 字节块
    fn process_block(&mut self, block: &[u8; 64]) {
        const S: [[u32; 4]; 4] = [
            [7, 12, 17, 22], [5, 9, 14, 20], [4, 11, 16, 23], [6, 10, 15, 21],
        ];
        const K: [u32; 64] = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
            0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
            0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
            0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
            0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
            0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
            0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
            0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
            0xeb86d391,
        ];

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        let mut m = [0u32; 16];
        for (i, chunk) in block.chunks(4).enumerate() {
            m[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                48..=63 => (c ^ (b | !d), (7 * i) % 16),
                _ => unreachable!(),
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(K[i])
                    .wrapping_add(m[g])
                    .rotate_left(S[i / 16][i % 4]),
            );
            a = temp;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    /// 计算最终的哈希值
    pub fn finalize(mut self) -> [u8; 16] {
        let bit_len_bytes = self.bit_len.to_le_bytes();
        self.update(&[0x80]);
        while self.data.len() % 64 != 56 {
            self.update(&[0x00]);
        }
        self.update(&bit_len_bytes);

        let mut hash = [0u8; 16];
        for (i, &val) in self.state.iter().enumerate() {
            hash[i * 4..(i + 1) * 4].copy_from_slice(&val.to_le_bytes());
        }
        hash
    }
}

/// Generate md5 string
///
/// # Example
///
/// ```no_run
/// use light_tool::md5;
/// println!("md5 string: {}", md5::str("hello world"))
/// ```
pub fn str<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    let mut md5 = Md5::new();
    md5.update(input.as_ref());
    let digest = md5.finalize();
    digest.iter().map(|byte| format!("{:02x}", byte)).collect()
}

/// Generate md5 string with salt
///
/// # Example
///
/// ```no_run
/// use light_tool::md5;
/// println!("md5 string with salt: {}", md5::salt("123", "456"))
/// ```
pub fn salt<T>(input: T, salt: T) -> String
where
    T: AsRef<[u8]>,
{
    let mut combined = Vec::with_capacity(input.as_ref().len() + salt.as_ref().len());
    combined.extend_from_slice(input.as_ref());
    combined.extend_from_slice(salt.as_ref());
    str(combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            salt("123", "456"),
            "e10adc3949ba59abbe56e057f20f883e"
        );
        assert_eq!(
            str("123456"),
            "e10adc3949ba59abbe56e057f20f883e"
        );
        assert_eq!(
            str(b"hello world"),
            "5eb63bbbe01eeed093cb22bb8f5acdc3"
        );
        assert_eq!(
            str(b""),
            "d41d8cd98f00b204e9800998ecf8427e"
        );
    }
}