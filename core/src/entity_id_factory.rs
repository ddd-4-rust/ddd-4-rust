//! Factory to create entity identifiers.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityIdFactory`.

use crate::entity_id::EntityId;

/// Factory to create entity identifiers.
///
/// Java: `EntityIdFactory`
pub trait EntityIdFactory: Send + Sync {
    /// Verifies if the given type string is a valid one.
    ///
    /// Java: `containsType(String type) -> boolean`
    fn contains_type(&self, r#type: &str) -> bool;

    /// Determines if an identifier of the given type is valid.
    ///
    /// Java: `isValid(String type, String id) -> boolean`
    fn is_valid(&self, r#type: &str, id: &str) -> bool;

    /// Creates an entity id by type and string identifier.
    ///
    /// Java: `createEntityId(String type, String id) -> EntityId`
    fn create_entity_id(&self, r#type: &str, id: &str) -> Option<Box<dyn EntityId>>;
}
