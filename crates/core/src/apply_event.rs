//! `apply_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.ApplyEvent`.

use crate::AggregateError;

/// Typed event-application contract represented by Java's `@ApplyEvent` marker.
///
/// `ApplyEvent` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait ApplyEvent<E> {
    /// Applies one event or returns a structured handler failure.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn apply_event(&mut self, event: &E) -> Result<(), AggregateError>;
}
