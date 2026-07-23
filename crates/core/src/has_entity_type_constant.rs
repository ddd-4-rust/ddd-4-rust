//! `has_entity_type_constant` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.HasEntityTypeConstant`.

/// Contract replacing Java reflection over a public static `TYPE` constant.
///
/// `HasEntityTypeConstant` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait HasEntityTypeConstant {
    /// Stable entity type string.
    ///
    /// `ENTITY_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const ENTITY_TYPE: &'static str;
}
