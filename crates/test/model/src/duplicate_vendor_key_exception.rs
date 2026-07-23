//! `duplicate_vendor_key_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::VendorKey;
use thiserror::Error;

/// Business-rule failure raised when a vendor key already exists.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Vendor key already exists: {key}")]
/// `DuplicateVendorKeyException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DuplicateVendorKeyException {
    key: VendorKey,
}
impl DuplicateVendorKeyException {
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(key: VendorKey) -> Self {
        Self { key }
    }
    #[must_use]
    /// 执行 `key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn key(&self) -> &VendorKey {
        &self.key
    }
}
