//! `the_root_name_gen` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `TheRootName_GEN`.

/// Input configuration that generates `TheRootName`.
#[derive(Debug, Clone, Copy, Default)]
/// `TheRootNameGen` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct TheRootNameGen;
impl TheRootNameGen {
    /// Minimum accepted name length.
    ///
    /// `MIN_LENGTH` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const MIN_LENGTH: usize = 1;
    /// Maximum accepted name length.
    ///
    /// `MAX_LENGTH` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const MAX_LENGTH: usize = 100;
}
