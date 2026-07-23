//! `java_parity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Cargo-discovered JSON-B test-model parity harness.

#[path = "java_parity/architecture_test.rs"]
mod architecture_test;
#[path = "java_parity/vendor_example_test.rs"]
mod vendor_example_test;
