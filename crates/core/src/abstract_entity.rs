//! `abstract_entity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Base class for entities within an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AbstractEntity`.

use crate::entity_id::EntityId;

/// Base implementation for entities within an aggregate.
///
/// Java: `AbstractEntity<ID extends EntityId, PARENT_ID extends EntityId, PARENT extends Entity<PARENT_ID>>`
///
/// `AbstractEntity` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractEntity<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> {
    id: Option<Box<Id>>,
    parent_id: Option<Box<ParentId>>,
}

impl<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> AbstractEntity<Id, ParentId> {
    /// Creates a new abstract entity.
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Self {
        Self {
            id: None,
            parent_id: None,
        }
    }

    /// Sets the entity's identifier.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    pub fn set_id(&mut self, id: Box<Id>) {
        self.id = Some(id);
    }

    /// Returns the entity's identifier.
    ///
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn id(&self) -> Option<&Id> {
        self.id.as_ref().map(std::convert::AsRef::as_ref)
    }

    /// Sets the parent entity's identifier.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    pub fn set_parent_id(&mut self, parent_id: Box<ParentId>) {
        self.parent_id = Some(parent_id);
    }

    /// Returns the parent entity's identifier.
    ///
    /// 执行 `parent_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn parent_id(&self) -> Option<&ParentId> {
        self.parent_id.as_ref().map(std::convert::AsRef::as_ref)
    }
}

impl<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> Default for AbstractEntity<Id, ParentId> {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}
