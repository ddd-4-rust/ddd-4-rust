//! `encryption_key_version_unknown_exception_data` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `jaxb/src/main/java/org/fuin/ddd4j/jaxb/EncryptionKeyVersionUnknownExceptionData.java`.

/// Compatibility type backed by the private shared representation.
///
/// `EncryptionKeyVersionUnknownExceptionData` 是迁移兼容层公开的稳定类型别名。
/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
pub type EncryptionKeyVersionUnknownExceptionData =
    crate::compat::EncryptionKeyVersionUnknownExceptionData;
