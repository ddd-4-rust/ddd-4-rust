//! `ddd4j_utils` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Utility methods and constants.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Ddd4JUtils`.

use crate::event_type::EventType;

/// Utility methods and constants for DDD operations.
///
/// Java: `Ddd4JUtils`
///
/// `Ddd4JUtils` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct Ddd4JUtils;

impl Ddd4JUtils {
    /// Prefix for unique short identifiers.
    ///
    /// Java: `SHORT_ID_PREFIX = "DDD4J"`
    ///
    /// `SHORT_ID_PREFIX` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const SHORT_ID_PREFIX: &'static str = "DDD4J";

    /// Creates an Adler32 checksum based on event type names.
    ///
    /// Java: `calculateChecksum(Collection<EventType> eventTypes) -> long`
    ///
    /// 执行 `calculate_checksum` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn calculate_checksum(event_types: &[EventType]) -> u32 {
        let mut adler = adler32::Adler32::new();
        for et in event_types {
            adler.update(et.as_str().as_bytes());
        }
        adler.finish()
    }
}

// Minimal Adler32 implementation (std-only, no external crate)
mod adler32 {
    /// `MOD_ADLER` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const MOD_ADLER: u32 = 65521;

    /// `Adler32` 表示与同名 Java 类型对应的 Rust 领域对象。
    /// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
    pub struct Adler32 {
        a: u32,
        b: u32,
    }

    impl Adler32 {
        /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
        /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
        pub fn new() -> Self {
            Self { a: 1, b: 0 }
        }

        /// 执行 `update` 对应的领域行为，参数和返回值遵循当前类型公开契约。
        /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
        pub fn update(&mut self, data: &[u8]) {
            for &byte in data {
                self.a = (self.a + u32::from(byte)) % MOD_ADLER;
                self.b = (self.b + self.a) % MOD_ADLER;
            }
        }

        /// 执行 `finish` 对应的领域行为，参数和返回值遵循当前类型公开契约。
        /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
        pub fn finish(&self) -> u32 {
            (self.b << 16) | self.a
        }
    }
}
