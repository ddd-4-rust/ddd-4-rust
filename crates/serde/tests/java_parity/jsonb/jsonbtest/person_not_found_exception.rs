//! `person_not_found_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `jsonb/src/test/java/org/fuin/ddd4j/jsonbtest/PersonNotFoundException.java`.

/// Serializer-specific alias of the shared executable test model.
///
/// `PersonNotFoundException` 是迁移兼容层公开的稳定类型别名。
/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
pub type PersonNotFoundException = ddd_4_rust_test_model::PersonNotFoundException;
