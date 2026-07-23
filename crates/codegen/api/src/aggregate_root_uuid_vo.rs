//! `aggregate_root_uuid_vo` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `codegen/api/.../AggregateRootUuidVO.java`.

use crate::SimpleValueObject;

/// Input model for generating a UUID aggregate-root identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `AggregateRootUuidVo` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateRootUuidVo {
    /// Optional Rust module path. Empty means the call site module.
    ///
    /// 保存 `module` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub module: String,
    /// Generated type name.
    ///
    /// 保存 `name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub name: String,
    /// Business description used in generated documentation.
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
    /// Documentation/example UUID.
    ///
    /// 保存 `example` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub example: String,
}

impl AggregateRootUuidVo {
    /// Java's default example UUID.
    ///
    /// `DEFAULT_EXAMPLE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const DEFAULT_EXAMPLE: &'static str = "00000000-0000-0000-0000-000000000000";
}
