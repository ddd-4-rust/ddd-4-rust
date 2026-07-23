//! `architecture_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
/// Test model intentionally depends on Core plus external serialization primitives only.
///
/// `TEST_MODEL_BOUNDARY` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
pub const TEST_MODEL_BOUNDARY: &str = "test-model -> core + serde";
