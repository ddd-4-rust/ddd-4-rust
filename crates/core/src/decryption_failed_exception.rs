//! `decryption_failed_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.DecryptionFailedException`.

use thiserror::Error;

/// Raised when encrypted content cannot be decrypted.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message}")]
/// `DecryptionFailedException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DecryptionFailedException {
    message: String,
}

impl DecryptionFailedException {
    /// Stable short identifier.
    ///
    /// `SHORT_ID` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const SHORT_ID: &'static str = "DDD4J-DECRYPTION_FAILED";
    /// Creates an exception with the Java-compatible message.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    /// Creates an exception from a cause message.
    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn from_cause(message: impl std::fmt::Display) -> Self {
        Self::new(format!("Decryption failed: {message}"))
    }
}
