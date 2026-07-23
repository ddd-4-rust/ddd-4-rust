//! `abstract_aggregate_root` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Base class for aggregate roots with event sourcing support.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AbstractAggregateRoot`.

use crate::aggregate_root_id::AggregateRootId;
use crate::aggregate_version::AggregateVersion;
use crate::domain_event::DomainEvent;
use crate::entity_id::EntityId;
use crate::exceptions::AggregateError;

/// Trait for aggregate roots that support applying events via dispatch.
///
/// Users implement this trait to provide the event handler dispatch logic.
/// The `#[apply_event]` proc macro will auto-generate this implementation.
///
/// Java: The `@ApplyEvent` annotation + reflection in `callAnnotatedEventHandlerMethod`
///
/// `ApplyEventHandler` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait ApplyEventHandler {
    /// Try to apply an event to this aggregate or its child entities.
    ///
    /// Returns `true` if the event was handled, `false` otherwise.
    ///
    /// Java: `callAnnotatedEventHandlerMethod(Entity<?> entity, DomainEvent<?> event)`
    ///
    /// 执行 `try_apply_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn try_apply_event(
        &mut self,
        event: &dyn DomainEvent<dyn EntityId>,
    ) -> Result<bool, AggregateError>;

    /// Returns a list of event types that should be ignored during history replay.
    ///
    /// Java: `getIgnoredEvents()`
    ///
    /// 执行 `ignored_events` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn ignored_events(&self) -> &[std::any::TypeId] {
        &[]
    }
}

/// Base implementation for aggregate roots.
///
/// Manages version, uncommitted changes, and event history replay.
/// Users embed this in their aggregate struct and delegate to it.
///
/// Java: `AbstractAggregateRoot<ID extends AggregateRootId>`
///
/// # Usage
///
/// ```ignore
/// struct MyAggregate {
///     root: AbstractAggregateRoot<PersonId>,
///     // ... fields
/// }
///
/// impl ApplyEventHandler for MyAggregate {
///     fn try_apply_event(
///         &mut self,
///         event: &dyn DomainEvent<dyn EntityId>,
///     ) -> Result<bool, AggregateError> {
///         // match on event type and apply
///         Ok(true)
///     }
/// }
///
/// impl MyAggregate {
///     fn do_something(&mut self) {
///         let event = ...;
///         self.root.apply(self, Box::new(event))?;
///         Ok::<(), AggregateError>(())
///     }
/// }
/// ```
///
/// `AbstractAggregateRoot` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractAggregateRoot<ID: AggregateRootId + ?Sized> {
    version: i32,
    uncommitted_changes: Vec<Box<dyn DomainEvent<dyn EntityId>>>,
    _phantom: std::marker::PhantomData<ID>,
}

impl<ID: AggregateRootId + ?Sized> AbstractAggregateRoot<ID> {
    /// Creates a new abstract aggregate root with version -1 (new, unsaved).
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Self {
        Self {
            version: -1,
            uncommitted_changes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the uncommitted changes.
    ///
    /// 执行 `uncommitted_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>] {
        &self.uncommitted_changes
    }

    /// Returns whether there are uncommitted changes.
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn has_uncommitted_changes(&self) -> bool {
        !self.uncommitted_changes.is_empty()
    }

    /// Clears the internal change list and sets the new version number.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    pub fn mark_changes_as_committed(&mut self) {
        self.version = self.next_version();
        self.uncommitted_changes.clear();
    }

    /// Returns the current version.
    ///
    /// 执行 `version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Returns the next version (current + uncommitted count).
    ///
    /// 执行 `next_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn next_version(&self) -> i32 {
        self.version
            .saturating_add(i32::try_from(self.uncommitted_changes.len()).unwrap_or(i32::MAX))
    }

    /// Returns the next apply version (next_version + 1).
    ///
    /// Java: `getNextApplyVersion()`
    ///
    /// 执行 `next_apply_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn next_apply_version(&self) -> AggregateVersion {
        AggregateVersion::new(
            u32::try_from(self.next_version().saturating_add(1)).unwrap_or_default(),
        )
    }

    /// Applies a new event to the aggregate.
    ///
    /// CAUTION: Don't use this method for applying historic events!
    /// Use `load_from_history` for replaying historic events.
    ///
    /// The `handler` should be the aggregate itself (which implements `ApplyEventHandler`).
    ///
    /// Java: `protected final void apply(@NotNull DomainEvent<?> event)`
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn apply(
        &mut self,
        handler: &mut dyn ApplyEventHandler,
        event: Box<dyn DomainEvent<dyn EntityId>>,
    ) -> Result<(), AggregateError> {
        if handler.try_apply_event(event.as_ref())? {
            self.uncommitted_changes.push(event);
            Ok(())
        } else {
            Err(AggregateError::EventHandlerNotFound {
                event_type: event.event_type().to_string(),
            })
        }
    }

    /// Loads the aggregate with historic events.
    ///
    /// The `handler` should be the aggregate itself (which implements `ApplyEventHandler`).
    ///
    /// Java: `loadFromHistory(DomainEvent<?>... history)` / `loadFromHistory(List<DomainEvent<?>> history)`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn load_from_history(
        &mut self,
        handler: &mut dyn ApplyEventHandler,
        history: &[Box<dyn DomainEvent<dyn EntityId>>],
    ) -> Result<(), AggregateError> {
        let ignored: Vec<std::any::TypeId> = handler.ignored_events().to_vec();
        for event in history {
            if !ignored.contains(&event.as_ref().type_id()) {
                let applied = handler.try_apply_event(event.as_ref())?;
                if applied {
                    self.version += 1;
                } else {
                    return Err(AggregateError::EventHandlerNotFound {
                        event_type: event.event_type().to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl<ID: AggregateRootId + ?Sized> Default for AbstractAggregateRoot<ID> {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}
