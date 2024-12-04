# LightTool

> 一个轻量级的Rust工具库，提倡的核心理念是：基于Rust原生库、无第三方依赖

## 快速开始

例如 `seconds()` 方法（获取当前秒级时间戳），使用方法如下所示

```rust
use light_tool::timestamp;

println!("{}", timestamp::seconds());
```

## 方法列表

+ **timestamp::seconds()** 秒级时间戳
+ **timestamp::milli_seconds()** 毫秒时间戳
+ **timestamp::nano_seconds()** 纳秒时间戳
+ **random::random_str()** 随机字符串
+ **random::random_num()** 随机数字
+ **random::random_alpha()** 随机字母
+ **random::random_alpha_num()** 随机字母数字
+ **random::random_range()** 指定范围内的随机数 [最小值, 最大值)
