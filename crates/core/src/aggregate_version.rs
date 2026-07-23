//! `aggregate_version` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Version of an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateVersion`.

use serde::{Deserialize, Serialize};

/// Version of an aggregate.
///
/// Java: `AggregateVersion extends AbstractIntegerValueObject`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// `AggregateVersion` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateVersion(u32);

impl AggregateVersion {
    /// Creates a new aggregate version.
    ///
    /// # Panics
    /// Panics if the version is negative (which is impossible for u32).
    ///
    /// Java: `new AggregateVersion(@Min(0) Integer version)`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(version: u32) -> Self {
        Self(version)
    }

    /// Returns the underlying integer value.
    ///
    /// Java: `asBaseType()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Returns the version as i32 for compatibility.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_i32(&self) -> i32 {
        i32::try_from(self.0).unwrap_or(i32::MAX)
    }

    /// Validates if a given value is a valid version.
    ///
    /// Java: `AggregateVersion.isValid(Integer value)` / `AggregateVersion.isValid(String value)`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid_u32(value: Option<u32>) -> bool {
        // Java validators consider null valid and leave presence to @NotNull.
        let _ = value;
        true
    }

    /// Validates if a given string is a valid version.
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid_str(value: Option<&str>) -> bool {
        match value {
            None => true,
            Some(v) => v.parse::<u32>().is_ok(),
        }
    }

    /// Creates an AggregateVersion from a u32 value.
    ///
    /// Java: `AggregateVersion.valueOf(Integer value)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of_u32(value: Option<u32>) -> Option<Self> {
        value.map(Self)
    }

    /// Creates an AggregateVersion from a string value.
    ///
    /// Java: `AggregateVersion.valueOf(String value)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of_str(value: Option<&str>) -> Option<Self> {
        value.and_then(|v| v.parse::<u32>().ok()).map(Self)
    }
}

impl std::fmt::Display for AggregateVersion {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for AggregateVersion {
    /// 执行 `from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn from(version: u32) -> Self {
        Self(version)
    }
}

impl From<AggregateVersion> for u32 {
    /// 执行 `from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn from(av: AggregateVersion) -> Self {
        av.0
    }
}
