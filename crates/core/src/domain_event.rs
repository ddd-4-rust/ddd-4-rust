//! `domain_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Domain event published by an entity.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.DomainEvent`.

use crate::aggregate_version::AggregateVersion;
use crate::entity_id::EntityId;
use crate::entity_id_path::EntityIdPath;
use crate::event::Event;

/// Domain event published by an entity.
///
/// Java: `DomainEvent<ID extends EntityId> extends Event`
///
/// Methods:
/// - `getEntityIdPath()` → `entity_id_path()`
/// - `getEntityId()` → `entity_id()` (the last ID in the path)
/// - `getAggregateVersion()` → `aggregate_version()`
/// - `getAggregateVersionInteger()` → `aggregate_version_integer()`
///
/// `DomainEvent` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait DomainEvent<ID: EntityId + ?Sized>: Event {
    /// Returns the path to the originator of the event.
    ///
    /// Java: `@NotNull getEntityIdPath()`
    ///
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath;

    /// Returns the identifier of the entity that caused this event (last ID in the path).
    ///
    /// Java: `@NotNull getEntityId()`
    ///
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &ID;

    /// Returns the version of the aggregate the entity belongs to.
    ///
    /// Java: `@Nullable getAggregateVersion()`
    ///
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion>;

    /// Returns the aggregate version as an integer.
    /// Null-safe shortcut for `aggregate_version().map(|v| v.as_u32())`.
    ///
    /// Java: `@Nullable getAggregateVersionInteger()`
    ///
    /// 执行 `aggregate_version_integer` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version_integer(&self) -> Option<u32> {
        self.aggregate_version().map(AggregateVersion::as_u32)
    }
}
