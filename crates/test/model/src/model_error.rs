//! `model_error` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{EntityIdPathError, EventTypeError};
use thiserror::Error;

/// Construction failure for the compatibility domain model.
#[derive(Debug, Error)]
/// `ModelError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum ModelError {
    /// A mandatory builder field was not set.
    #[error("missing mandatory builder field: {0}")]
    MissingBuilderField(&'static str),
    /// An event type did not satisfy Core validation.
    #[error(transparent)]
    EventType(#[from] EventTypeError),
    /// An entity path did not satisfy Core validation.
    #[error(transparent)]
    EntityPath(#[from] EntityIdPathError),
}
