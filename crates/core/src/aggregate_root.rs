//! `aggregate_root` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Dedicated entity that guarantees consistency within an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateRoot`.

use crate::aggregate_root_id::AggregateRootId;
use crate::aggregate_version::AggregateVersion;
use crate::domain_event::DomainEvent;
use crate::entity::Entity;
use crate::entity_id::EntityId;
use crate::exceptions::AggregateError;

/// Dedicated entity of a group of entities (The group is called "Aggregate")
/// that guarantees the consistency of changes being made within the group
/// by forbidding external objects from holding direct references to its members.
///
/// Java: `AggregateRoot<ID extends AggregateRootId> extends Entity<ID>`
///
/// `AggregateRoot` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait AggregateRoot<ID: AggregateRootId + ?Sized>: Entity<ID> {
    /// Returns a list of uncommitted changes.
    ///
    /// Java: `@NotNull getUncommittedChanges() -> List<DomainEvent<?>>`
    ///
    /// 执行 `uncommitted_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>];

    /// Returns whether the aggregate has uncommitted changes.
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn has_uncommitted_changes(&self) -> bool {
        !self.uncommitted_changes().is_empty()
    }

    /// Clears the internal change list and sets the new version number.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn mark_changes_as_committed(&mut self);

    /// Returns the current version of the aggregate (excluding uncommitted changes).
    ///
    /// 执行 `version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn version(&self) -> i32;

    /// Returns the next version of the aggregate (including uncommitted changes).
    ///
    /// 执行 `next_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn next_version(&self) -> i32 {
        self.version()
            .saturating_add(i32::try_from(self.uncommitted_changes().len()).unwrap_or(i32::MAX))
    }

    /// Returns the next version useful when creating an event for being applied.
    ///
    /// 执行 `next_apply_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn next_apply_version(&self) -> AggregateVersion {
        AggregateVersion::new(
            u32::try_from(self.next_version().saturating_add(1)).unwrap_or_default(),
        )
    }

    /// Loads the aggregate with historic events.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn load_from_history(
        &mut self,
        history: &[Box<dyn DomainEvent<dyn EntityId>>],
    ) -> Result<(), AggregateError>;

    /// Applies a new event to the aggregate.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn apply(&mut self, event: Box<dyn DomainEvent<dyn EntityId>>) -> Result<(), AggregateError>;
}
