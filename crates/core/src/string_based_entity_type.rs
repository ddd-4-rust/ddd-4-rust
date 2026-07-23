//! `string_based_entity_type` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.StringBasedEntityType`.

use crate::EntityType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Validation failures for [`StringBasedEntityType`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// `EntityTypeError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum EntityTypeError {
    /// The type name was empty.
    #[error("entity type must not be empty")]
    Empty,
    /// The type name exceeded the supported length.
    #[error("entity type must not exceed {max} characters, got {actual}")]
    TooLong {
        /// Maximum supported length.
        max: usize,
        /// Actual length.
        actual: usize,
    },
}

/// Entity type based on a non-empty string of at most 255 bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// `StringBasedEntityType` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct StringBasedEntityType(String);

impl StringBasedEntityType {
    /// Maximum serialized length.
    ///
    /// `MAX_LENGTH` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const MAX_LENGTH: usize = 255;

    /// Validates and creates an entity type.
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(value: impl Into<String>) -> Result<Self, EntityTypeError> {
        let value = value.into();
        if value.is_empty() {
            return Err(EntityTypeError::Empty);
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(EntityTypeError::TooLong {
                max: Self::MAX_LENGTH,
                actual: value.len(),
            });
        }
        Ok(Self(value))
    }
}

impl EntityType for StringBasedEntityType {
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StringBasedEntityType {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for StringBasedEntityType {
    /// `Error` 是迁移兼容层公开的稳定类型别名。
    /// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
    type Error = EntityTypeError;

    /// 执行 `try_from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for StringBasedEntityType {
    /// `Error` 是迁移兼容层公开的稳定类型别名。
    /// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
    type Error = EntityTypeError;

    /// 执行 `try_from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
