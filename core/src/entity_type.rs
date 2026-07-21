//! Identifies a type of entity within all entity types of the context.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityType` and `StringBasedEntityType`.

use serde::{Deserialize, Serialize};

/// Identifies a type of entity within all entity types of the context.
///
/// Java: `EntityType extends Serializable`
pub trait EntityType: std::fmt::Display + Send + Sync {
    /// Returns the entity type name as string.
    ///
    /// Java: `asString()`
    fn as_string(&self) -> &str;
}

/// Entity type based on a string with a maximum length of 255 characters.
///
/// Java: `StringBasedEntityType extends AbstractStringValueObject implements EntityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringBasedEntityType(String);

impl StringBasedEntityType {
    /// Maximum length of the entity type string.
    pub const MAX_LENGTH: usize = 255;

    /// Creates a new entity type.
    ///
    /// # Panics
    /// Panics if the string is empty or longer than 255 characters.
    ///
    /// Java: `new StringBasedEntityType(@NotEmpty @Size(max = 255) String str)`
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "EntityType must not be empty");
        assert!(
            value.len() <= Self::MAX_LENGTH,
            "EntityType must not exceed {} characters, got {}",
            Self::MAX_LENGTH,
            value.len()
        );
        Self(value)
    }
}

impl EntityType for StringBasedEntityType {
    fn as_string(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StringBasedEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for StringBasedEntityType {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for StringBasedEntityType {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
