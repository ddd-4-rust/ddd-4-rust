//! `integer_entity_id_vo` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `codegen/api/.../IntegerEntityIdVO.java`.

use crate::SimpleValueObject;

/// Input model for generating an integer entity identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `IntegerEntityIdVo` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct IntegerEntityIdVo {
    /// Optional Rust module path.
    ///
    /// 保存 `module` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub module: String,
    /// Generated type name.
    ///
    /// 保存 `name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub name: String,
    /// Business description.
    ///
    /// 保存 `description` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub description: String,
    /// Stable wire-level entity type.
    ///
    /// 保存 `entity_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_type: String,
    /// Shared persistence and serialization switches.
    ///
    /// 保存 `targets` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub targets: SimpleValueObject,
    /// Generate `OpenAPI` metadata.
    ///
    /// 保存 `openapi` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub openapi: bool,
    /// Inclusive minimum value.
    ///
    /// 保存 `min_value` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub min_value: i32,
    /// Inclusive maximum value.
    ///
    /// 保存 `max_value` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub max_value: i32,
}

impl IntegerEntityIdVo {
    /// Java annotation default for the lower bound.
    ///
    /// `DEFAULT_MIN_VALUE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const DEFAULT_MIN_VALUE: i32 = 1;
    /// Java annotation default for the upper bound.
    ///
    /// `DEFAULT_MAX_VALUE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const DEFAULT_MAX_VALUE: i32 = i32::MAX;
}
