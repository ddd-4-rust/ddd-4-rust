//! An ordered list of entity identifiers.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityIdPath`.

use crate::entity_id::EntityId;
use crate::entity_id_factory::EntityIdFactory;
use std::sync::Arc;

/// The separator between entity identifiers in serialized form.
pub const PATH_SEPARATOR: &str = "/";

/// An ordered list of entity identifiers.
///
/// An aggregate root will be the first entry if it's contained in the list.
///
/// Uses `Arc<dyn EntityId>` for shared ownership, matching Java's reference semantics
/// in `rest()` and `parent()` methods.
///
/// Serialized as its string representation.
///
/// Java: `EntityIdPath extends AbstractStringValueObject implements Serializable`
#[derive(Debug, Clone)]
pub struct EntityIdPath {
    entity_ids: Vec<Arc<dyn EntityId>>,
}

impl EntityIdPath {
    /// Creates a new entity identifier path from a list of entity IDs.
    ///
    /// # Panics
    /// Panics if the list is empty or contains null elements.
    ///
    /// Java: `new EntityIdPath(EntityId... entityIds)` / `new EntityIdPath(List<EntityId> ids)`
    pub fn new(entity_ids: Vec<Arc<dyn EntityId>>) -> Self {
        assert!(!entity_ids.is_empty(), "Identifier list cannot be empty");
        Self { entity_ids }
    }

    /// Returns an iterator over the entity identifiers.
    ///
    /// Java: `iterator()`
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn EntityId>> {
        self.entity_ids.iter()
    }

    /// Returns the number of entity IDs.
    pub fn len(&self) -> usize {
        self.entity_ids.len()
    }

    /// Returns true if the path is empty (never — constructor prevents this).
    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }

    /// Returns the first entity identifier in the path.
    ///
    /// Java: `first() -> <T extends EntityId> T`
    pub fn first(&self) -> &Arc<dyn EntityId> {
        &self.entity_ids[0]
    }

    /// Returns the last entity identifier in the path.
    ///
    /// Java: `last() -> <T extends EntityId> T`
    pub fn last(&self) -> &Arc<dyn EntityId> {
        &self.entity_ids[self.entity_ids.len() - 1]
    }

    /// Returns the path without the first entry, or None if only one element.
    ///
    /// Java: `rest() -> EntityIdPath` (returns null if only one element)
    pub fn rest(&self) -> Option<EntityIdPath> {
        if self.entity_ids.len() <= 1 {
            return None;
        }
        Some(EntityIdPath {
            entity_ids: self.entity_ids[1..].to_vec(),
        })
    }

    /// Returns the parent path without the last entry, or None if only one element.
    ///
    /// Java: `parent() -> EntityIdPath` (returns null if only one element)
    pub fn parent(&self) -> Option<EntityIdPath> {
        if self.entity_ids.len() <= 1 {
            return None;
        }
        Some(EntityIdPath {
            entity_ids: self.entity_ids[..self.entity_ids.len() - 1].to_vec(),
        })
    }

    /// Returns the number of elements in the path.
    ///
    /// Java: `size()`
    pub fn size(&self) -> usize {
        self.entity_ids.len()
    }

    /// Returns the serialized form of the path.
    ///
    /// Java: `asBaseType()` / `toString()`
    pub fn as_base_type(&self) -> String {
        self.entity_ids
            .iter()
            .map(|id| id.as_typed_string())
            .collect::<Vec<_>>()
            .join(PATH_SEPARATOR)
    }

    /// Converts a string into an EntityIdPath using the given factory.
    ///
    /// Java: `EntityIdPath.valueOf(EntityIdFactory factory, String str)`
    pub fn value_of(factory: &dyn EntityIdFactory, str: Option<&str>) -> Option<Self> {
        let str = str?;
        let entries: Vec<&str> = str.split(PATH_SEPARATOR).collect();
        if entries.is_empty() {
            return None;
        }
        let mut ids: Vec<Arc<dyn EntityId>> = Vec::new();
        for entry in entries {
            // Parse "Type id" format
            if let Some(p) = entry.find(' ') {
                let r#type = &entry[..p];
                let id = &entry[p + 1..];
                ids.push(Arc::from(factory.create_entity_id(r#type, id)?));
            } else {
                return None;
            }
        }
        Some(Self { entity_ids: ids })
    }

    /// Validates that a string is a valid entity identifier path.
    ///
    /// Java: `EntityIdPath.isValid(EntityIdFactory factory, String value)`
    pub fn is_valid(factory: &dyn EntityIdFactory, value: Option<&str>) -> bool {
        let value = match value {
            None => return true,
            Some(v) => v,
        };
        if value.is_empty() {
            return false;
        }
        let entries: Vec<&str> = value.split(PATH_SEPARATOR).collect();
        if entries.is_empty() {
            return false;
        }
        for entry in entries {
            if let Some(p) = entry.find(' ') {
                let r#type = &entry[..p];
                let id = &entry[p + 1..];
                if !factory.contains_type(r#type) || !factory.is_valid(r#type, id) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for EntityIdPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_base_type())
    }
}

// Serialize as the string representation (asBaseType)
impl serde::Serialize for EntityIdPath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_base_type().serialize(serializer)
    }
}

// Deserialize from string using an EntityIdFactory.
// NOTE: Deserialization requires an EntityIdFactory, so this is a limited implementation
// that works only when the factory is globally available or for known entity ID types.
impl<'de> serde::Deserialize<'de> for EntityIdPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _s = String::deserialize(deserializer)?;
        // For now, deserialization is limited. In practice, a custom deserializer
        // with access to EntityIdFactory should be used.
        Err(serde::de::Error::custom(
            "EntityIdPath deserialization requires EntityIdFactory. Use a custom deserializer.",
        ))
    }
}
