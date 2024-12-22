use std::cell::UnsafeCell;
use std::sync::Once;

pub struct Lazy<T> {
    init: fn() -> T,
    once: Once,
    value: UnsafeCell<Option<T>>,
}

// `Sync` 的实现：让 `Lazy` 可以安全地跨线程访问
unsafe impl<T> Sync for Lazy<T> {}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Lazy {
            init,
            once: Once::new(),
            value: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> &T {
        // 确保初始化只执行一次
        self.once.call_once(|| {
            let value = (self.init)();
            unsafe {
                *self.value.get() = Some(value);
            }
        });

        // 安全地返回初始化后的值
        unsafe {
            (*self.value.get())
                .as_ref()
                .expect("Lazy value should have been initialized")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy() {
        static GLOBAL_DATA: Lazy<u64> = Lazy::new(|| {
            println!("init...");
            42
        });

        println!("value: {}", GLOBAL_DATA.get()); // 第一次访问时会打印 "init..."
        println!("value: {}", GLOBAL_DATA.get()); // 后续访问不会重复初始化
    }
}