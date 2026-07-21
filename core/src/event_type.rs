//! Identifies an event type within an aggregate type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EventType`.

use serde::{Deserialize, Serialize};

/// Identifies an event type within an aggregate type.
///
/// Java: `EventType extends AbstractStringValueObject`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventType(String);

impl EventType {
    /// Maximum length of the event type string.
    pub const MAX_LENGTH: usize = 255;

    /// Creates a new event type.
    ///
    /// # Panics
    /// Panics if the string is empty or longer than 255 characters.
    ///
    /// Java: `new EventType(String str)` with `@NotEmpty @Size(max = 255)`
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "EventType must not be empty");
        assert!(
            value.len() <= Self::MAX_LENGTH,
            "EventType must not exceed {} characters, got {}",
            Self::MAX_LENGTH,
            value.len()
        );
        Self(value)
    }

    /// Returns the underlying string.
    ///
    /// Java: `asBaseType()`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for EventType {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for EventType {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<EventType> for String {
    fn from(et: EventType) -> Self {
        et.0
    }
}
