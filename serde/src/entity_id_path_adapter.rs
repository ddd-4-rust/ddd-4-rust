//! Custom serde serializer/deserializer for `EntityIdPath`.
//!
//! 1:1 translation of `EntityIdPathJacksonDeserializer`.

use ddd_4_rust_core::{EntityIdFactory, EntityIdPath};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize EntityIdPath as its `as_base_type()` string representation.
pub fn serialize_entity_id_path<S: Serializer>(
    path: &EntityIdPath,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    path.to_string().serialize(serializer)
}

/// Deserialize EntityIdPath from string, given an EntityIdFactory.
///
/// Java: `EntityIdPath.valueOf(EntityIdFactory factory, String str)`
pub fn deserialize_entity_id_path_with_factory<'de, D: Deserializer<'de>>(
    factory: &dyn EntityIdFactory,
    deserializer: D,
) -> Result<EntityIdPath, D::Error> {
    let s = String::deserialize(deserializer)?;
    EntityIdPath::value_of(factory, Some(&s))
        .ok_or_else(|| serde::de::Error::custom(format!("Invalid EntityIdPath: {}", s)))
}
