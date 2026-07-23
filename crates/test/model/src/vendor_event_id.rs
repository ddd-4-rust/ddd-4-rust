//! `vendor_event_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::VendorId;
use serde::{Deserialize, Serialize};

/// Composite vendor-stream event identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// `VendorEventId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorEventId {
    vendor_id: String,
    event_number: i32,
}
impl VendorEventId {
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(vendor_id: &VendorId, event_number: i32) -> Self {
        Self {
            vendor_id: vendor_id.to_string(),
            event_number,
        }
    }
    #[must_use]
    /// 执行 `vendor_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn vendor_id(&self) -> &str {
        &self.vendor_id
    }
    #[must_use]
    /// 执行 `event_number` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn event_number(&self) -> i32 {
        self.event_number
    }
}
impl std::fmt::Display for VendorEventId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}-{}", self.vendor_id, self.event_number)
    }
}
