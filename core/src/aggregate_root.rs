//! Dedicated entity that guarantees consistency within an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateRoot`.

use crate::aggregate_root_id::AggregateRootId;
use crate::aggregate_version::AggregateVersion;
use crate::domain_event::DomainEvent;
use crate::entity::Entity;
use crate::entity_id::EntityId;

/// Dedicated entity of a group of entities (The group is called "Aggregate")
/// that guarantees the consistency of changes being made within the group
/// by forbidding external objects from holding direct references to its members.
///
/// Java: `AggregateRoot<ID extends AggregateRootId> extends Entity<ID>`
pub trait AggregateRoot<ID: AggregateRootId + ?Sized>: Entity<ID> {
    /// Returns a list of uncommitted changes.
    ///
    /// Java: `@NotNull getUncommittedChanges() -> List<DomainEvent<?>>`
    fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>];

    /// Returns whether the aggregate has uncommitted changes.
    fn has_uncommitted_changes(&self) -> bool {
        !self.uncommitted_changes().is_empty()
    }

    /// Clears the internal change list and sets the new version number.
    fn mark_changes_as_committed(&mut self);

    /// Returns the current version of the aggregate (excluding uncommitted changes).
    fn version(&self) -> i32;

    /// Returns the next version of the aggregate (including uncommitted changes).
    fn next_version(&self) -> i32 {
        self.version() + self.uncommitted_changes().len() as i32
    }

    /// Returns the next version useful when creating an event for being applied.
    fn next_apply_version(&self) -> AggregateVersion {
        AggregateVersion::new((self.next_version() + 1) as u32)
    }

    /// Loads the aggregate with historic events.
    fn load_from_history(&mut self, history: &[Box<dyn DomainEvent<dyn EntityId>>]);

    /// Applies a new event to the aggregate.
    fn apply(&mut self, event: Box<dyn DomainEvent<dyn EntityId>>);
}
