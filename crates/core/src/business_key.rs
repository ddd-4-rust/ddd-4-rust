//! `business_key` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Human-readable business (natural) key marker trait.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.BusinessKey`.

/// Human-readable business (natural) key often used in documents or shown in the user interface.
///
/// Java: `BusinessKey extends Serializable`
///
/// Marker trait for value objects that represent a business/natural key.
///
/// `BusinessKey` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait BusinessKey: Send + Sync + 'static {}
