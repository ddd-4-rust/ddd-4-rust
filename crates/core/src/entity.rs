//! `entity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! An object defined by its identity, not its attributes.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Entity`.

use crate::entity_id::EntityId;
use crate::entity_type::EntityType;

/// An object that is not defined by its attributes,
/// but rather by a thread of continuity and its identity.
///
/// Java: `Entity<ID extends EntityId>`
///
/// `Entity` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait Entity<ID: EntityId + ?Sized>: Send + Sync {
    /// Returns the unique type.
    ///
    /// Java: `getType() -> EntityType`
    ///
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType;

    /// Returns the unique entity identifier.
    ///
    /// Java: `getId() -> ID`
    ///
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn id(&self) -> &ID;
}
