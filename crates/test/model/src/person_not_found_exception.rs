//! `person_not_found_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{PersonId, VendorRef};
use thiserror::Error;

/// Raised when a person cannot be found inside a vendor aggregate.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Person {person_id} not found for {vendor_ref}")]
/// `PersonNotFoundException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct PersonNotFoundException {
    vendor_ref: VendorRef,
    person_id: PersonId,
}
impl PersonNotFoundException {
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(vendor_ref: VendorRef, person_id: PersonId) -> Self {
        Self {
            vendor_ref,
            person_id,
        }
    }
    #[must_use]
    /// 执行 `vendor_ref` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn vendor_ref(&self) -> &VendorRef {
        &self.vendor_ref
    }
    #[must_use]
    /// 执行 `person_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn person_id(&self) -> PersonId {
        self.person_id
    }
}
