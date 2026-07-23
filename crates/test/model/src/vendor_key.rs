//! `vendor_key` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::VendorKeyStrValidator;
use ddd_4_rust_core::BusinessKey;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("vendor key must match V[0-9]{{5}}")]
/// `VendorKeyError` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorKeyError;

/// Validated vendor business key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
/// `VendorKey` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorKey(String);
impl VendorKey {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(value: impl Into<String>) -> Result<Self, VendorKeyError> {
        let value = value.into();
        if Self::is_valid(Some(&value)) {
            Ok(Self(value))
        } else {
            Err(VendorKeyError)
        }
    }
    #[must_use]
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_str(&self) -> &str {
        &self.0
    }
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(value: Option<&str>) -> bool {
        VendorKeyStrValidator::is_valid(value)
    }
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of(value: Option<&str>) -> Result<Option<Self>, VendorKeyError> {
        value.map(Self::new).transpose()
    }
}
impl BusinessKey for VendorKey {}
impl std::fmt::Display for VendorKey {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
