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
//! | `IntegerEntityId` | [`IntegerEntityId`] |
//! | `AggregateCache<A>` | [`AggregateCache<A>`] trait |
//! | `BusinessKey` | [`BusinessKey`] trait |
//! | `EncryptedData` | [`EncryptedData`] trait |
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
pub mod integer_entity_id;
pub mod entity_id_factory;
pub mod repository;
pub mod exceptions;
pub mod aggregate_cache;
pub mod business_key;
pub mod ddd_utils;
pub mod exception_data;
pub mod encrypted_data;
pub mod encrypted_data_service;

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
pub use abstract_aggregate_root::{AbstractAggregateRoot, ApplyEventHandler};
pub use abstract_entity::AbstractEntity;
pub use aggregate_root_uuid::AggregateRootUuid;
pub use integer_entity_id::IntegerEntityId;
pub use entity_id_factory::EntityIdFactory;
pub use repository::Repository;
pub use exceptions::AggregateError;
pub use aggregate_cache::{AggregateCache, AggregateNoCache};
pub use business_key::BusinessKey;
pub use ddd_utils::Ddd4JUtils;
pub use exception_data::ExceptionData;
pub use encrypted_data::EncryptedData;
pub use encrypted_data_service::{EncryptedDataService, EncryptionError};
