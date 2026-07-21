//! An object defined by its identity, not its attributes.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Entity`.

use crate::entity_id::EntityId;
use crate::entity_type::EntityType;

/// An object that is not defined by its attributes,
/// but rather by a thread of continuity and its identity.
///
/// Java: `Entity<ID extends EntityId>`
pub trait Entity<ID: EntityId + ?Sized>: Send + Sync {
    /// Returns the unique type.
    ///
    /// Java: `getType() -> EntityType`
    fn entity_type(&self) -> &dyn EntityType;

    /// Returns the unique entity identifier.
    ///
    /// Java: `getId() -> ID`
    fn id(&self) -> &ID;
}
