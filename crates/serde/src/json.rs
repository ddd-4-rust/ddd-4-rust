//! `json` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Serde-native JSON support and frozen Java compatibility namespaces.

/// Primary Serde-native JSON API.
///
/// 公开 `serde` 子模块，用于组织对应领域类型及其兼容入口。
pub mod serde;

/// Frozen source-compatibility facade for callers migrating from Java names.
#[deprecated(
    since = "0.7.0",
    note = "use `ddd_4_rust_serde::json::serde`; this namespace only preserves the frozen Java migration API"
)]
/// 公开 `jackson` 子模块，用于组织对应领域类型及其兼容入口。
pub mod jackson;
/// 公开 `jsonb` 子模块，用于组织对应领域类型及其兼容入口。
pub mod jsonb;
