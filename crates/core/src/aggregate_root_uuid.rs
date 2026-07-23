//! `aggregate_root_uuid` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! UUID based aggregate root identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateRootUuid`.

use crate::aggregate_root_id::AggregateRootId;
use crate::entity_id::EntityId;
use crate::{EntityType, EntityTypeError, StringBasedEntityType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;

/// UUID based aggregate root identifier.
///
/// Concrete types should extend/use this struct and provide their own `EntityType` constant.
///
/// Java: `AggregateRootUuid implements AggregateRootId, Comparable<AggregateRootUuid>, ValueObjectWithBaseType<UUID>`
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// `AggregateRootUuid` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateRootUuid {
    entity_type: StringBasedEntityType,
    uuid: Uuid,
}

impl AggregateRootUuid {
    /// Creates a new random UUID-based aggregate root identifier.
    ///
    /// Java: `AggregateRootUuid(EntityType entityType)`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(entity_type: &str) -> Result<Self, EntityTypeError> {
        Ok(Self {
            entity_type: StringBasedEntityType::new(entity_type)?,
            uuid: Uuid::new_v4(),
        })
    }

    /// Creates from a given entity type and UUID.
    ///
    /// Java: `AggregateRootUuid(EntityType entityType, UUID uuid)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn from_uuid(entity_type: &str, uuid: Uuid) -> Result<Self, EntityTypeError> {
        Ok(Self {
            entity_type: StringBasedEntityType::new(entity_type)?,
            uuid,
        })
    }

    /// Returns the UUID.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Validates a UUID string.
    ///
    /// Java: `AggregateRootUuid.isValid(String value)`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(value: Option<&str>) -> bool {
        match value {
            None => true,
            Some(v) => {
                if v.len() != 36 {
                    return false;
                }
                Uuid::parse_str(v).is_ok()
            }
        }
    }
}

impl Debug for AggregateRootUuid {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AggregateRootUuid({} {})",
            self.entity_type.as_string(),
            self.uuid
        )
    }
}

impl Display for AggregateRootUuid {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl EntityId for AggregateRootUuid {
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType {
        &self.entity_type
    }

    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> String {
        self.uuid.to_string()
    }

    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type.as_string(), self.uuid)
    }
}

impl AggregateRootId for AggregateRootUuid {}

impl Ord for AggregateRootUuid {
    /// 执行 `cmp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity_type
            .as_string()
            .cmp(other.entity_type.as_string())
            .then_with(|| self.uuid.cmp(&other.uuid))
    }
}

impl PartialOrd for AggregateRootUuid {
    /// 执行 `partial_cmp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
