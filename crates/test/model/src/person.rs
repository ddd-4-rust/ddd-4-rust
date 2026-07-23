//! `person` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{ModelError, PersonId, PersonName, PersonNameChangedEvent, VendorRef};
use ddd_4_rust_core::AggregateVersion;

/// Person child entity inside a vendor aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `Person` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct Person {
    id: PersonId,
    name: PersonName,
}
impl Person {
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(id: PersonId, name: PersonName) -> Self {
        Self { id, name }
    }
    #[must_use]
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn id(&self) -> PersonId {
        self.id
    }
    #[must_use]
    /// 执行 `name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn name(&self) -> &PersonName {
        &self.name
    }
    /// 执行 `change_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn change_name(
        &mut self,
        vendor_ref: VendorRef,
        new_name: PersonName,
        version: AggregateVersion,
    ) -> Result<PersonNameChangedEvent, ModelError> {
        let event = PersonNameChangedEvent::new(
            vendor_ref,
            self.id,
            self.name.clone(),
            new_name.clone(),
            version,
        )?;
        self.name = new_name;
        Ok(event)
    }
}
