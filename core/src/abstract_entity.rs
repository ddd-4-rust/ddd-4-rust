//! Base class for entities within an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AbstractEntity`.

use crate::entity_id::EntityId;

/// Base implementation for entities within an aggregate.
///
/// Java: `AbstractEntity<ID extends EntityId, PARENT_ID extends EntityId, PARENT extends Entity<PARENT_ID>>`
pub struct AbstractEntity<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> {
    id: Option<Box<Id>>,
    parent_id: Option<Box<ParentId>>,
}

impl<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> AbstractEntity<Id, ParentId> {
    /// Creates a new abstract entity.
    pub fn new() -> Self {
        Self {
            id: None,
            parent_id: None,
        }
    }

    /// Sets the entity's identifier.
    pub fn set_id(&mut self, id: Box<Id>) {
        self.id = Some(id);
    }

    /// Returns the entity's identifier.
    pub fn id(&self) -> Option<&Id> {
        self.id.as_ref().map(|id| id.as_ref())
    }

    /// Sets the parent entity's identifier.
    pub fn set_parent_id(&mut self, parent_id: Box<ParentId>) {
        self.parent_id = Some(parent_id);
    }

    /// Returns the parent entity's identifier.
    pub fn parent_id(&self) -> Option<&ParentId> {
        self.parent_id.as_ref().map(|id| id.as_ref())
    }
}

impl<Id: EntityId + ?Sized, ParentId: EntityId + ?Sized> Default
    for AbstractEntity<Id, ParentId>
{
    fn default() -> Self {
        Self::new()
    }
}
