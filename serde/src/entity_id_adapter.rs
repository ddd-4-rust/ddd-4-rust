//! Custom serde serializer/deserializer for `dyn EntityId`.
//!
//! 1:1 translation of `EntityIdJacksonSerializer` + `EntityIdJacksonDeserializer`.

use ddd_4_rust_core::EntityId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Serialize a `dyn EntityId` as its `as_typed_string()` representation.
///
/// Java: `EntityIdJacksonSerializer`
pub fn serialize_entity_id<S: Serializer>(
    entity_id: &dyn EntityId,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    entity_id.as_typed_string().serialize(serializer)
}

/// Deserialize a `dyn EntityId` from its `as_typed_string()` representation.
///
/// NOTE: Requires an EntityIdFactory to resolve type+id strings.
/// This function is a helper that returns the raw string for factory resolution.
///
/// Java: `EntityIdJacksonDeserializer`
pub fn deserialize_entity_id_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    String::deserialize(deserializer)
}

/// Helper to serialize an `Option<&dyn EntityId>`.
pub fn serialize_entity_id_opt<S: Serializer>(
    entity_id: &Option<&dyn EntityId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match entity_id {
        Some(id) => serialize_entity_id(*id, serializer),
        None => serializer.serialize_none(),
    }
}
