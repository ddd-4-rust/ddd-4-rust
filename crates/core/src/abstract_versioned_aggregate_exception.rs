//! `abstract_versioned_aggregate_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.AbstractVersionedAggregateException`.

use crate::AbstractAggregateException;

/// Shared immutable data of version-related aggregate failures.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `AbstractVersionedAggregateException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractVersionedAggregateException {
    aggregate: AbstractAggregateException,
    version: i32,
}

impl AbstractVersionedAggregateException {
    /// Creates versioned aggregate failure data.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        message: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
        version: i32,
    ) -> Self {
        Self {
            aggregate: AbstractAggregateException::new(message, entity_type, entity_id),
            version,
        }
    }
    /// Aggregate failure data.
    #[must_use]
    /// 执行 `aggregate` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn aggregate(&self) -> &AbstractAggregateException {
        &self.aggregate
    }
    /// Associated aggregate version.
    #[must_use]
    /// 执行 `version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn version(&self) -> i32 {
        self.version
    }
}

impl std::fmt::Display for AbstractVersionedAggregateException {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.aggregate.fmt(formatter)
    }
}

impl std::error::Error for AbstractVersionedAggregateException {}
