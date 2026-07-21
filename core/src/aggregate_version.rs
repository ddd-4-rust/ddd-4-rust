//! Version of an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateVersion`.

use serde::{Deserialize, Serialize};

/// Version of an aggregate.
///
/// Java: `AggregateVersion extends AbstractIntegerValueObject`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AggregateVersion(u32);

impl AggregateVersion {
    /// Creates a new aggregate version.
    ///
    /// # Panics
    /// Panics if the version is negative (which is impossible for u32).
    ///
    /// Java: `new AggregateVersion(@Min(0) Integer version)`
    pub fn new(version: u32) -> Self {
        Self(version)
    }

    /// Returns the underlying integer value.
    ///
    /// Java: `asBaseType()`
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Returns the version as i32 for compatibility.
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }

    /// Validates if a given value is a valid version.
    ///
    /// Java: `AggregateVersion.isValid(Integer value)` / `AggregateVersion.isValid(String value)`
    pub fn is_valid_u32(value: Option<u32>) -> bool {
        // All u32 values >= 0, so always valid
        value.is_some()
    }

    /// Validates if a given string is a valid version.
    pub fn is_valid_str(value: Option<&str>) -> bool {
        match value {
            None => true,
            Some(v) => v.parse::<u32>().is_ok(),
        }
    }

    /// Creates an AggregateVersion from a u32 value.
    ///
    /// Java: `AggregateVersion.valueOf(Integer value)`
    pub fn value_of_u32(value: Option<u32>) -> Option<Self> {
        value.map(Self)
    }

    /// Creates an AggregateVersion from a string value.
    ///
    /// Java: `AggregateVersion.valueOf(String value)`
    pub fn value_of_str(value: Option<&str>) -> Option<Self> {
        value.and_then(|v| v.parse::<u32>().ok()).map(Self)
    }
}

impl std::fmt::Display for AggregateVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for AggregateVersion {
    fn from(version: u32) -> Self {
        Self(version)
    }
}

impl From<AggregateVersion> for u32 {
    fn from(av: AggregateVersion) -> Self {
        av.0
    }
}
