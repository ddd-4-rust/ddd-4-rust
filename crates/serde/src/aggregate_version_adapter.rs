//! `aggregate_version_adapter` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Custom serde serializer/deserializer for `AggregateVersion`.
//!
//! 1:1 translation of `AggregateVersionJacksonSerializer` + `AggregateVersionJacksonDeserializer`.

use ddd_4_rust_core::AggregateVersion;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize `AggregateVersion` as u32.
///
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn serialize_aggregate_version<S: Serializer>(
    version: &AggregateVersion,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    version.as_u32().serialize(serializer)
}

/// Deserialize `AggregateVersion` from u32.
///
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn deserialize_aggregate_version<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<AggregateVersion, D::Error> {
    let v = u32::deserialize(deserializer)?;
    Ok(AggregateVersion::new(v))
}

/// Serialize `Option<AggregateVersion>`.
///
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn serialize_aggregate_version_opt<S: Serializer>(
    version: &Option<AggregateVersion>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match version {
        Some(v) => serialize_aggregate_version(v, serializer),
        None => serializer.serialize_none(),
    }
}

/// Deserialize `Option<AggregateVersion>`.
///
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn deserialize_aggregate_version_opt<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<AggregateVersion>, D::Error> {
    Option::<u32>::deserialize(deserializer).map(|opt| opt.map(AggregateVersion::new))
}
