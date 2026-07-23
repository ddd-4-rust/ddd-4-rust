//! `entity_type` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Identifies a type of entity within all entity types of the context.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityType`.

/// Identifies a type of entity within all entity types of the context.
///
/// Java: `EntityType extends Serializable`
///
/// `EntityType` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EntityType: std::fmt::Display + Send + Sync {
    /// Returns the entity type name as string.
    ///
    /// Java: `asString()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> &str;
}
