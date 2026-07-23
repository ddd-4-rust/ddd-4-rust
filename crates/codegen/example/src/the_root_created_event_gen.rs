//! `the_root_created_event_gen` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `TheRootCreatedEvent_GEN`.

/// Input configuration that generates `TheRootCreatedEvent`.
#[derive(Debug, Clone, Copy, Default)]
/// `TheRootCreatedEventGen` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct TheRootCreatedEventGen;
impl TheRootCreatedEventGen {
    /// Stable generated event type.
    ///
    /// `EVENT_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const EVENT_TYPE: &'static str = "TheRootCreatedEvent";
}
