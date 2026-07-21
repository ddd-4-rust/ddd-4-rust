//! Universal unique event identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EventId`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Universal unique event identifier.
///
/// Java: `EventId extends AbstractUuidValueObject implements TechnicalId, AsStringCapable, Serializable`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(Uuid);

impl EventId {
    /// Creates a new random event identifier.
    ///
    /// Java: `new EventId()`
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an event identifier from a UUID.
    ///
    /// Java: `new EventId(UUID uuid)`
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying UUID.
    ///
    /// Java: `asBaseType()`
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Returns the string representation.
    ///
    /// Java: `asString()`
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Converts a string into an event identifier.
    ///
    /// Java: `EventId.valueOf(String value)`
    pub fn value_of(value: &str) -> Option<Self> {
        Uuid::parse_str(value).ok().map(Self)
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for EventId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<EventId> for Uuid {
    fn from(event_id: EventId) -> Self {
        event_id.0
    }
}
