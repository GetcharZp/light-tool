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
+ **random::str()** 随机字符串
+ **random::num()** 随机数字
+ **random::alpha()** 随机字母
+ **random::alpha_num()** 随机字母数字
+ **random::range()** 指定范围内的随机数 [最小值, 最大值)
+ **md5::str()** 获取MD5值
+ **md5::salt()** 获取加盐后的MD5值
+ **mac::address()** 获取Mac地址
+ **uuid::new()** UUID
+ **http::get()** GET 请求
+ **http::post()** POST 请求
+ **http::put()** PUT 请求
+ **http::delete()** DELETE 请求
+ **file::copy()** 拷贝文件
+ **file::rename()** 移动文件
+ **file::create_parent_dir()** 创建目标文件的父目录
