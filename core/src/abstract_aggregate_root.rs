//! Base class for aggregate roots with event sourcing support.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AbstractAggregateRoot`.

use crate::aggregate_root_id::AggregateRootId;
use crate::aggregate_version::AggregateVersion;
use crate::domain_event::DomainEvent;
use crate::entity_id::EntityId;

/// Trait for aggregate roots that support applying events via dispatch.
///
/// Users implement this trait to provide the event handler dispatch logic.
/// The `#[apply_event]` proc macro will auto-generate this implementation.
///
/// Java: The `@ApplyEvent` annotation + reflection in `callAnnotatedEventHandlerMethod`
pub trait ApplyEventHandler {
    /// Try to apply an event to this aggregate or its child entities.
    ///
    /// Returns `true` if the event was handled, `false` otherwise.
    ///
    /// Java: `callAnnotatedEventHandlerMethod(Entity<?> entity, DomainEvent<?> event)`
    fn try_apply_event(&mut self, event: &dyn DomainEvent<dyn EntityId>) -> bool;

    /// Returns a list of event types that should be ignored during history replay.
    ///
    /// Java: `getIgnoredEvents()`
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
///     fn try_apply_event(&mut self, event: &dyn DomainEvent<dyn EntityId>) -> bool {
///         // match on event type and apply
///         true
///     }
/// }
///
/// impl MyAggregate {
///     fn do_something(&mut self) {
///         let event = ...;
///         self.root.apply(self, Box::new(event));
///     }
/// }
/// ```
pub struct AbstractAggregateRoot<ID: AggregateRootId + ?Sized> {
    version: i32,
    uncommitted_changes: Vec<Box<dyn DomainEvent<dyn EntityId>>>,
    _phantom: std::marker::PhantomData<ID>,
}

impl<ID: AggregateRootId + ?Sized> AbstractAggregateRoot<ID> {
    /// Creates a new abstract aggregate root with version -1 (new, unsaved).
    pub fn new() -> Self {
        Self {
            version: -1,
            uncommitted_changes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the uncommitted changes.
    pub fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>] {
        &self.uncommitted_changes
    }

    /// Returns whether there are uncommitted changes.
    pub fn has_uncommitted_changes(&self) -> bool {
        !self.uncommitted_changes.is_empty()
    }

    /// Clears the internal change list and sets the new version number.
    pub fn mark_changes_as_committed(&mut self) {
        self.version = self.next_version();
        self.uncommitted_changes.clear();
    }

    /// Returns the current version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Returns the next version (current + uncommitted count).
    pub fn next_version(&self) -> i32 {
        self.version + self.uncommitted_changes.len() as i32
    }

    /// Returns the next apply version (next_version + 1).
    ///
    /// Java: `getNextApplyVersion()`
    pub fn next_apply_version(&self) -> AggregateVersion {
        AggregateVersion::new((self.next_version() + 1) as u32)
    }

    /// Applies a new event to the aggregate.
    ///
    /// CAUTION: Don't use this method for applying historic events!
    /// Use `load_from_history` for replaying historic events.
    ///
    /// The `handler` should be the aggregate itself (which implements `ApplyEventHandler`).
    ///
    /// Java: `protected final void apply(@NotNull DomainEvent<?> event)`
    pub fn apply(
        &mut self,
        handler: &mut dyn ApplyEventHandler,
        event: Box<dyn DomainEvent<dyn EntityId>>,
    ) {
        if handler.try_apply_event(event.as_ref()) {
            self.uncommitted_changes.push(event);
        } else {
            panic!(
                "Couldn't find an event handler for event type: {:?}",
                event.event_type()
            );
        }
    }

    /// Loads the aggregate with historic events.
    ///
    /// The `handler` should be the aggregate itself (which implements `ApplyEventHandler`).
    ///
    /// Java: `loadFromHistory(DomainEvent<?>... history)` / `loadFromHistory(List<DomainEvent<?>> history)`
    pub fn load_from_history(
        &mut self,
        handler: &mut dyn ApplyEventHandler,
        history: &[Box<dyn DomainEvent<dyn EntityId>>],
    ) {
        let ignored: Vec<std::any::TypeId> = handler.ignored_events().to_vec();
        for event in history {
            if !ignored.contains(&event.as_ref().type_id()) {
                let applied = handler.try_apply_event(event.as_ref());
                if applied {
                    self.version += 1;
                } else {
                    panic!(
                        "Wasn't able to apply historic event '{:?}' to aggregate",
                        event.event_type()
                    );
                }
            }
        }
    }
}

impl<ID: AggregateRootId + ?Sized> Default for AbstractAggregateRoot<ID> {
    fn default() -> Self {
        Self::new()
    }
}
