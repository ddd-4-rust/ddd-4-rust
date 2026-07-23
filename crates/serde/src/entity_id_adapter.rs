//! `entity_id_adapter` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Custom serde serializer/deserializer for `dyn EntityId`.
//!
//! 1:1 translation of `EntityIdJacksonSerializer` + `EntityIdJacksonDeserializer`.

use ddd_4_rust_core::EntityId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize a `dyn EntityId` as its `as_typed_string()` representation.
///
/// Java: `EntityIdJacksonSerializer`
///
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn serialize_entity_id<S: Serializer>(
    entity_id: &dyn EntityId,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    entity_id.as_typed_string().serialize(serializer)
}

/// Deserialize a `dyn EntityId` from its `as_typed_string()` representation.
///
/// NOTE: Requires an `EntityIdFactory` to resolve type+id strings.
/// This function is a helper that returns the raw string for factory resolution.
///
/// Java: `EntityIdJacksonDeserializer`
///
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn deserialize_entity_id_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    String::deserialize(deserializer)
}

/// Helper to serialize an `Option<&dyn EntityId>`.
///
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn serialize_entity_id_opt<S: Serializer>(
    entity_id: &Option<&dyn EntityId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match entity_id {
        Some(id) => serialize_entity_id(*id, serializer),
        None => serializer.serialize_none(),
    }
}
