//! `xml` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! XML compatibility namespace.

/// 公开 `jaxb` 子模块，用于组织对应领域类型及其兼容入口。
pub mod jaxb;
