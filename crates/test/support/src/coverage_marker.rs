//! `coverage_marker` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.jacoco.Dummy`.

/// Real coverage-pipeline marker replacing the empty Java `JaCoCo` module.
///
/// `CoverageMarker` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct CoverageMarker;
impl CoverageMarker {
    /// Environment variable written by LLVM source-based coverage runs.
    ///
    /// `PROFILE_ENV` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const PROFILE_ENV: &'static str = "LLVM_PROFILE_FILE";
    /// Returns whether the current process was launched with an LLVM coverage profile target.
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_enabled() -> bool {
        std::env::var_os(Self::PROFILE_ENV).is_some()
    }
}
