//! DDD-4-Rust Serde: serde-based serialization for DDD building blocks.
//!
//! 1:1 translation of `ddd-4-java-jackson` (merged with jsonb).
//!
//! Provides `AbstractEvent` and `AbstractDomainEvent` base structs with serde support,
//! plus custom serde adapters for EntityId, EntityIdPath, AggregateVersion.

mod abstract_event;
mod abstract_domain_event;
mod entity_id_adapter;
mod entity_id_path_adapter;
mod aggregate_version_adapter;
mod ddd_serde_module;

pub use abstract_event::AbstractEvent;
pub use abstract_domain_event::AbstractDomainEvent;
pub use entity_id_adapter::{deserialize_entity_id_string, serialize_entity_id, serialize_entity_id_opt};
pub use entity_id_path_adapter::{deserialize_entity_id_path_with_factory, serialize_entity_id_path};
pub use aggregate_version_adapter::{
    deserialize_aggregate_version, deserialize_aggregate_version_opt,
    serialize_aggregate_version, serialize_aggregate_version_opt,
};
pub use ddd_serde_module::DddSerdeModule;
