//! `vendor_ref` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{VendorId, VendorKey, VendorName};
use serde::{Deserialize, Serialize};

/// Stable reference to a vendor aggregate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[expect(
    clippy::struct_field_names,
    reason = "field names mirror the Java model"
)]
/// `VendorRef` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorRef {
    #[serde(rename = "id")]
    vendor_id: VendorId,
    #[serde(rename = "key")]
    vendor_key: VendorKey,
    #[serde(rename = "name")]
    vendor_name: VendorName,
}
impl VendorRef {
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(vendor_id: VendorId, vendor_key: VendorKey, vendor_name: VendorName) -> Self {
        Self {
            vendor_id,
            vendor_key,
            vendor_name,
        }
    }
    #[must_use]
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn id(&self) -> &VendorId {
        &self.vendor_id
    }
    #[must_use]
    /// 执行 `key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn key(&self) -> &VendorKey {
        &self.vendor_key
    }
    #[must_use]
    /// 执行 `name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn name(&self) -> &VendorName {
        &self.vendor_name
    }
}
impl std::fmt::Display for VendorRef {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Vendor {}", self.vendor_key)
    }
}
