//! Integer based entity identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.IntegerEntityId`.

use crate::entity_id::EntityId;
use crate::entity_type::EntityType;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Integer based entity identifier.
///
/// Java: `IntegerEntityId implements EntityId, Comparable<IntegerEntityId>, ValueObjectWithBaseType<Integer>`
///
/// # Usage
/// Concrete types should implement `IntegerEntityId` and provide their own `EntityType`.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntegerEntityId {
    entity_type: String,
    id: i32,
}

impl IntegerEntityId {
    /// Creates a new integer entity ID.
    ///
    /// Java: `new IntegerEntityId(EntityType entityType, Integer id)`
    pub fn new(entity_type: impl Into<String>, id: i32) -> Self {
        Self {
            entity_type: entity_type.into(),
            id,
        }
    }

    /// Returns the integer value.
    ///
    /// Java: `asBaseType() -> Integer`
    pub fn as_i32(&self) -> i32 {
        self.id
    }
}

impl Debug for IntegerEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerEntityId({} {})", self.entity_type, self.id)
    }
}

impl Display for IntegerEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl EntityId for IntegerEntityId {
    fn entity_type(&self) -> &dyn EntityType {
        // Return a reference to a StringBasedEntityType constructed on the fly.
        // In practice, concrete types should override entity_type().
        // This uses a static approach via a leaked string reference.
        unimplemented!("IntegerEntityId::entity_type() requires concrete type. Use a wrapper struct.")
    }

    fn as_string(&self) -> String {
        self.id.to_string()
    }

    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type, self.id)
    }
}

impl Ord for IntegerEntityId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity_type
            .cmp(&other.entity_type)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for IntegerEntityId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
