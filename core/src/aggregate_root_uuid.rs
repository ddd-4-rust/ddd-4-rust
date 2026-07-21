//! UUID based aggregate root identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateRootUuid`.

use crate::aggregate_root_id::AggregateRootId;
use crate::entity_id::EntityId;
use crate::entity_type::EntityType;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;

/// UUID based aggregate root identifier.
///
/// Concrete types should extend/use this struct and provide their own `EntityType` constant.
///
/// Java: `AggregateRootUuid implements AggregateRootId, Comparable<AggregateRootUuid>, ValueObjectWithBaseType<UUID>`
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AggregateRootUuid {
    entity_type: StringBasedEntityType,
    uuid: Uuid,
}

impl AggregateRootUuid {
    /// Creates a new random UUID-based aggregate root identifier.
    ///
    /// Java: `AggregateRootUuid(EntityType entityType)`
    pub fn new(entity_type: &str) -> Self {
        Self {
            entity_type: StringBasedEntityType::new(entity_type),
            uuid: Uuid::new_v4(),
        }
    }

    /// Creates from a given entity type and UUID.
    ///
    /// Java: `AggregateRootUuid(EntityType entityType, UUID uuid)`
    pub fn from_uuid(entity_type: &str, uuid: Uuid) -> Self {
        Self {
            entity_type: StringBasedEntityType::new(entity_type),
            uuid,
        }
    }

    /// Returns the UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Validates a UUID string.
    ///
    /// Java: `AggregateRootUuid.isValid(String value)`
    pub fn is_valid(value: Option<&str>) -> bool {
        match value {
            None => true,
            Some(v) => {
                if v.len() != 36 {
                    return false;
                }
                Uuid::parse_str(v).is_ok()
            }
        }
    }
}

impl Debug for AggregateRootUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AggregateRootUuid({} {})", self.entity_type.as_string(), self.uuid)
    }
}

impl Display for AggregateRootUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl EntityId for AggregateRootUuid {
    fn entity_type(&self) -> &dyn EntityType {
        &self.entity_type
    }

    fn as_string(&self) -> String {
        self.uuid.to_string()
    }

    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type.as_string(), self.uuid)
    }
}

impl AggregateRootId for AggregateRootUuid {}

impl Ord for AggregateRootUuid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity_type
            .as_string()
            .cmp(other.entity_type.as_string())
            .then_with(|| self.uuid.cmp(&other.uuid))
    }
}

impl PartialOrd for AggregateRootUuid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use crate::entity_type::StringBasedEntityType;
