//! DDD-4-Rust Core: Domain-Driven Design building blocks for Rust.
//!
//! This crate provides foundational DDD types and traits, translated 1:1 from `ddd-4-java-core`.
//!
//! # Key Types
//!
//! | Java | Rust |
//! |------|------|
//! | `Event` | [`Event`] trait |
//! | `DomainEvent<ID>` | [`DomainEvent<ID>`] trait |
//! | `EntityId` | [`EntityId`] trait |
//! | `AggregateRootId` | [`AggregateRootId`] trait |
//! | `AggregateRoot<ID>` | [`AggregateRoot<ID>`] trait |
//! | `Repository<ID, T>` | [`Repository<ID, T>`] trait |
//! | `AbstractAggregateRoot<ID>` | [`AbstractAggregateRoot<ID>`] |
//! | `EventId` | [`EventId`] |
//! | `EventType` | [`EventType`] |
//! | `EntityType` | [`EntityType`] trait |
//! | `AggregateVersion` | [`AggregateVersion`] |
//! | `EntityIdPath` | [`EntityIdPath`] |

pub mod event_id;
pub mod event_type;
pub mod entity_type;
pub mod entity_id;
pub mod aggregate_root_id;
pub mod aggregate_version;
pub mod entity_id_path;
pub mod event;
pub mod domain_event;
pub mod entity;
pub mod aggregate_root;
pub mod abstract_aggregate_root;
pub mod abstract_entity;
pub mod aggregate_root_uuid;
pub mod entity_id_factory;
pub mod repository;
pub mod exceptions;

// Re-export everything
pub use event_id::EventId;
pub use event_type::EventType;
pub use entity_type::{EntityType, StringBasedEntityType};
pub use entity_id::EntityId;
pub use aggregate_root_id::AggregateRootId;
pub use aggregate_version::AggregateVersion;
pub use entity_id_path::EntityIdPath;
pub use event::Event;
pub use domain_event::DomainEvent;
pub use entity::Entity;
pub use aggregate_root::AggregateRoot;
pub use abstract_aggregate_root::AbstractAggregateRoot;
pub use abstract_entity::AbstractEntity;
pub use aggregate_root_uuid::AggregateRootUuid;
pub use entity_id_factory::EntityIdFactory;
pub use repository::Repository;
pub use exceptions::AggregateError;
