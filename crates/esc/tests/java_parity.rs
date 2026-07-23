//! `java_parity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Cargo-discovered harness for all Java ESC compatibility files.

#[path = "java_parity/esc/aggregate_stream_id_test.rs"]
mod aggregate_stream_id_test;
#[path = "java_parity/esc/architecture_test.rs"]
mod architecture_test;
#[path = "java_parity/esc/base_test.rs"]
mod base_test;
#[path = "java_parity/esc/event_store_respository_test.rs"]
mod event_store_respository_test;
#[path = "java_parity/esc/vendor_repository.rs"]
mod vendor_repository;
