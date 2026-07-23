//! `integer_entity_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Integer based entity identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.IntegerEntityId`.

use crate::entity_id::EntityId;
use crate::{EntityType, EntityTypeError, StringBasedEntityType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Integer based entity identifier.
///
/// Java: `IntegerEntityId implements EntityId, Comparable<IntegerEntityId>, ValueObjectWithBaseType<Integer>`
///
/// # Usage
/// Concrete types should implement `IntegerEntityId` and provide their own `EntityType`.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// `IntegerEntityId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct IntegerEntityId {
    entity_type: StringBasedEntityType,
    id: i32,
}

impl IntegerEntityId {
    /// Creates a new integer entity ID.
    ///
    /// Java: `new IntegerEntityId(EntityType entityType, Integer id)`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(entity_type: impl Into<String>, id: i32) -> Result<Self, EntityTypeError> {
        Ok(Self {
            entity_type: StringBasedEntityType::new(entity_type)?,
            id,
        })
    }

    /// Returns the integer value.
    ///
    /// Java: `asBaseType() -> Integer`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_i32(&self) -> i32 {
        self.id
    }
}

impl Debug for IntegerEntityId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerEntityId({} {})", self.entity_type, self.id)
    }
}

impl Display for IntegerEntityId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl EntityId for IntegerEntityId {
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType {
        &self.entity_type
    }

    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> String {
        self.id.to_string()
    }

    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type.as_string(), self.id)
    }
}

impl Ord for IntegerEntityId {
    /// 执行 `cmp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity_type
            .cmp(&other.entity_type)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for IntegerEntityId {
    /// 执行 `partial_cmp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
