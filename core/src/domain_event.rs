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
pub trait DomainEvent<ID: EntityId + ?Sized>: Event {
    /// Returns the path to the originator of the event.
    ///
    /// Java: `@NotNull getEntityIdPath()`
    fn entity_id_path(&self) -> &EntityIdPath;

    /// Returns the identifier of the entity that caused this event (last ID in the path).
    ///
    /// Java: `@NotNull getEntityId()`
    fn entity_id(&self) -> &ID;

    /// Returns the version of the aggregate the entity belongs to.
    ///
    /// Java: `@Nullable getAggregateVersion()`
    fn aggregate_version(&self) -> Option<&AggregateVersion>;

    /// Returns the aggregate version as an integer.
    /// Null-safe shortcut for `aggregate_version().map(|v| v.as_u32())`.
    ///
    /// Java: `@Nullable getAggregateVersionInteger()`
    fn aggregate_version_integer(&self) -> Option<u32> {
        self.aggregate_version().map(|v| v.as_u32())
    }
}
