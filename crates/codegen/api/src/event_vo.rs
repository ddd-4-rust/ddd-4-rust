//! `event_vo` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `codegen/api/.../EventVO.java`.

use crate::SimpleValueObject;

/// Input model for generating a domain event value object.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `EventVo` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EventVo {
    /// Optional Rust module path.
    ///
    /// 保存 `module` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub module: String,
    /// Generated event type name.
    ///
    /// 保存 `name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub name: String,
    /// Entity identifier type path.
    ///
    /// 保存 `entity_id_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_id_type: String,
    /// Rust expression used to construct the entity ID path.
    ///
    /// 保存 `entity_id_path_parameters` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_id_path_parameters: String,
    /// Business description.
    ///
    /// 保存 `description` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub description: String,
    /// Serialization targets. The Java event annotation does not use `jpa`.
    ///
    /// 保存 `targets` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub targets: SimpleValueObject,
    /// Generate `OpenAPI` metadata.
    ///
    /// 保存 `openapi` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub openapi: bool,
    /// Display message template.
    ///
    /// 保存 `message` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub message: String,
}
