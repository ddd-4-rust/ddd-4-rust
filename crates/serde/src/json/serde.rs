//! `serde` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Serde-native JSON adapters for DDD value objects and wire data.
//!
//! This is the primary JSON API. It uses `serde` traits and `serde_json` wire
//! values directly; no Java serializer runtime or registration module exists.

use ddd_4_rust_core::{AggregateVersion, EntityId, EntityIdFactory, EntityIdPath};

pub use crate::compat::{
    AbstractAggregateExceptionData, AbstractVersionedAggregateExceptionData,
    AggregateAlreadyExistsExceptionData, AggregateDeletedExceptionData,
    AggregateNotFoundExceptionData, AggregateVersionConflictExceptionData,
    AggregateVersionNotFoundExceptionData, DecryptionFailedExceptionData,
    DuplicateEncryptionKeyIdExceptionData, DuplicateEntityExceptionData,
    EncryptionKeyIdUnknownExceptionData, EncryptionKeyVersionUnknownExceptionData,
    EntityNotFoundExceptionData,
};
pub use crate::{
    AbstractDomainEvent, AbstractEvent, DddSerdeModule, ZonedDateTimeError, ZonedDateTimeValue,
    deserialize_aggregate_version, deserialize_aggregate_version_opt,
    deserialize_entity_id_path_with_factory, deserialize_entity_id_string,
    serialize_aggregate_version, serialize_aggregate_version_opt, serialize_entity_id,
    serialize_entity_id_opt, serialize_entity_id_path,
};

/// Serde-backed encrypted wire data.
///
/// `EncryptedData` 是迁移兼容层公开的稳定类型别名。
/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
pub type EncryptedData = crate::compat::CompatEncryptedData;

/// Convenience adapter for aggregate versions outside derive attributes.
///
/// `AggregateVersionAdapter` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateVersionAdapter;

impl AggregateVersionAdapter {
    /// Converts an aggregate version to its JSON number representation.
    #[must_use]
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn serialize(value: &AggregateVersion) -> u32 {
        value.as_u32()
    }

    /// Creates an aggregate version from its JSON number representation.
    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn deserialize(value: u32) -> AggregateVersion {
        AggregateVersion::new(value)
    }
}

/// Convenience adapter for typed entity IDs outside derive attributes.
///
/// `EntityIdAdapter` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityIdAdapter;

impl EntityIdAdapter {
    /// Converts an entity ID to its stable typed string.
    #[must_use]
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn serialize(value: &dyn EntityId) -> String {
        value.as_typed_string()
    }

    /// Recreates an entity ID using the domain's registered factory.
    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn deserialize(
        factory: &dyn EntityIdFactory,
        value: Option<&str>,
    ) -> Option<Box<dyn EntityId>> {
        value
            .and_then(|value| {
                value
                    .find(' ')
                    .map(|position| (&value[..position], &value[position + 1..]))
            })
            .and_then(|(kind, id)| factory.create_entity_id(kind, id))
    }
}

/// Convenience adapter for typed entity-ID paths outside derive attributes.
///
/// `EntityIdPathAdapter` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityIdPathAdapter;

impl EntityIdPathAdapter {
    /// Converts an entity-ID path to its stable string representation.
    #[must_use]
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn serialize(value: Option<&EntityIdPath>) -> Option<String> {
        value.map(EntityIdPath::as_base_type)
    }

    /// Recreates an entity-ID path using the domain's registered factory.
    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn deserialize(factory: &dyn EntityIdFactory, value: Option<&str>) -> Option<EntityIdPath> {
        EntityIdPath::value_of(factory, value)
    }
}
