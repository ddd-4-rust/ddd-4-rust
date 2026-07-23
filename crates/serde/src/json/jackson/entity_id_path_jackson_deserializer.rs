//! `entity_id_path_jackson_deserializer` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `jackson/src/main/java/org/fuin/ddd4j/jackson/EntityIdPathJacksonDeserializer.java`.

/// Legacy name delegating to the Serde-native entity-ID-path adapter.
///
/// `EntityIdPathJacksonDeserializer` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityIdPathJacksonDeserializer;
impl EntityIdPathJacksonDeserializer {
    #[must_use]
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn marshal(value: Option<&ddd_4_rust_core::EntityIdPath>) -> Option<String> {
        crate::json::serde::EntityIdPathAdapter::serialize(value)
    }

    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn unmarshal(
        factory: &dyn ddd_4_rust_core::EntityIdFactory,
        value: Option<&str>,
    ) -> Option<ddd_4_rust_core::EntityIdPath> {
        crate::json::serde::EntityIdPathAdapter::deserialize(factory, value)
    }
}
