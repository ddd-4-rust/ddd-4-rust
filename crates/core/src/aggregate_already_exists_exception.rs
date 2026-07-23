//! `aggregate_already_exists_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.AggregateAlreadyExistsException`.

use crate::AbstractVersionedAggregateException;
use thiserror::Error;

/// Raised when adding an aggregate whose stream already exists.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{data}")]
/// `AggregateAlreadyExistsException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateAlreadyExistsException {
    data: AbstractVersionedAggregateException,
}

impl AggregateAlreadyExistsException {
    /// Stable short identifier.
    ///
    /// `SHORT_ID` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const SHORT_ID: &'static str = "DDD4J-AGGREGATE_ALREADY_EXISTS";
    /// Creates the exception.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(entity_type: impl Into<String>, entity_id: impl Into<String>, version: i32) -> Self {
        let entity_type = entity_type.into();
        let entity_id = entity_id.into();
        let message = format!("{entity_type} {entity_id} already exists (version={version})");
        Self {
            data: AbstractVersionedAggregateException::new(
                message,
                entity_type,
                entity_id,
                version,
            ),
        }
    }
    /// Shared versioned data.
    #[must_use]
    /// 执行 `data` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn data(&self) -> &AbstractVersionedAggregateException {
        &self.data
    }
}
