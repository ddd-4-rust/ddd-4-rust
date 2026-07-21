//! Custom serde serializer/deserializer for `AggregateVersion`.
//!
//! 1:1 translation of `AggregateVersionJacksonSerializer` + `AggregateVersionJacksonDeserializer`.

use ddd_4_rust_core::AggregateVersion;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize AggregateVersion as u32.
pub fn serialize_aggregate_version<S: Serializer>(
    version: &AggregateVersion,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    version.as_u32().serialize(serializer)
}

/// Deserialize AggregateVersion from u32.
pub fn deserialize_aggregate_version<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<AggregateVersion, D::Error> {
    let v = u32::deserialize(deserializer)?;
    Ok(AggregateVersion::new(v))
}

/// Serialize Option<AggregateVersion>.
pub fn serialize_aggregate_version_opt<S: Serializer>(
    version: &Option<AggregateVersion>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match version {
        Some(v) => serialize_aggregate_version(v, serializer),
        None => serializer.serialize_none(),
    }
}

/// Deserialize Option<AggregateVersion>.
pub fn deserialize_aggregate_version_opt<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<AggregateVersion>, D::Error> {
    Option::<u32>::deserialize(deserializer).map(|opt| opt.map(AggregateVersion::new))
}
