//! Unique name of an aggregate stream.
//!
//! 1:1 translation of `org.fuin.ddd4j.esc.AggregateStreamId`.

use ddd_4_rust_core::aggregate_root_id::AggregateRootId;
use ddd_4_rust_core::entity_type::EntityType;

/// Unique name of an aggregate stream.
///
/// Java: `AggregateStreamId implements StreamId`
#[derive(Debug, Clone)]
pub struct AggregateStreamId {
    entity_type: String,
    param_name: String,
    param_value: String,
}

impl AggregateStreamId {
    /// Creates a new aggregate stream ID.
    ///
    /// Java: `new AggregateStreamId(EntityType type, String paramName, AggregateRootId paramValue)`
    pub fn new(
        entity_type: &dyn EntityType,
        param_name: &str,
        param_value: &dyn AggregateRootId,
    ) -> Self {
        Self {
            entity_type: entity_type.as_string().to_string(),
            param_name: param_name.to_string(),
            param_value: param_value.as_string(),
        }
    }

    /// Returns the stream name.
    ///
    /// Java: `getName() -> String`
    pub fn name(&self) -> &str {
        &self.entity_type
    }

    /// Returns whether this is a projection stream.
    ///
    /// Java: `isProjection() -> boolean`
    pub fn is_projection(&self) -> bool {
        false
    }

    /// Returns the string representation: `{type}-{id}`.
    ///
    /// Java: `asString() -> String`
    pub fn as_string(&self) -> String {
        format!("{}-{}", self.entity_type, self.param_value)
    }

    /// Returns the parameter value as string.
    ///
    /// Java: `getSingleParamValue() -> <T> T`
    pub fn param_value(&self) -> &str {
        &self.param_value
    }
}

impl std::fmt::Display for AggregateStreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl PartialEq for AggregateStreamId {
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
    }
}

impl Eq for AggregateStreamId {}

impl std::hash::Hash for AggregateStreamId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_string().hash(state);
    }
}
