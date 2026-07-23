//! `entity_id_path_adapter` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Custom serde serializer/deserializer for `EntityIdPath`.
//!
//! 1:1 translation of `EntityIdPathJacksonDeserializer`.

use ddd_4_rust_core::{EntityIdFactory, EntityIdPath};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize `EntityIdPath` as its `as_base_type()` string representation.
///
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn serialize_entity_id_path<S: Serializer>(
    path: &EntityIdPath,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    path.to_string().serialize(serializer)
}

/// Deserialize `EntityIdPath` from string, given an `EntityIdFactory`.
///
/// Java: `EntityIdPath.valueOf(EntityIdFactory factory, String str)`
///
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn deserialize_entity_id_path_with_factory<'de, D: Deserializer<'de>>(
    factory: &dyn EntityIdFactory,
    deserializer: D,
) -> Result<EntityIdPath, D::Error> {
    let s = String::deserialize(deserializer)?;
    EntityIdPath::value_of(factory, Some(&s))
        .ok_or_else(|| serde::de::Error::custom(format!("Invalid EntityIdPath: {s}")))
}
