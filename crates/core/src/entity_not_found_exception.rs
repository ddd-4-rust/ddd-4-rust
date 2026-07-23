//! `entity_not_found_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.EntityNotFoundException`.

use thiserror::Error;

/// Raised when an entity cannot be found below an optional parent path.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message}")]
/// `EntityNotFoundException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityNotFoundException {
    parent_id_path: Option<String>,
    entity_id: String,
    message: String,
}

impl EntityNotFoundException {
    /// Stable short identifier.
    ///
    /// `SHORT_ID` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const SHORT_ID: &'static str = "DDD4J-ENTITY_NOT_FOUND";
    /// Creates the exception.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(parent_id_path: Option<String>, entity_id: impl Into<String>) -> Self {
        let entity_id = entity_id.into();
        let message = parent_id_path.as_ref().map_or_else(
            || format!("{entity_id} not found"),
            |parent| format!("{entity_id} not found in {parent}"),
        );
        Self {
            parent_id_path,
            entity_id,
            message,
        }
    }
    /// Optional parent path.
    #[must_use]
    /// 执行 `parent_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn parent_id_path(&self) -> Option<&str> {
        self.parent_id_path.as_deref()
    }
    /// Missing entity identifier.
    #[must_use]
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }
}
