//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
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

#![expect(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::doc_markdown,
    reason = "the public surface mirrors the frozen Java 0.7.0 API; documentation is completed file-by-file during migration"
)]

/// 公开 `abstract_aggregate_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod abstract_aggregate_exception;
/// 公开 `abstract_aggregate_root` 子模块，用于组织对应领域类型及其兼容入口。
pub mod abstract_aggregate_root;
/// 公开 `abstract_entity` 子模块，用于组织对应领域类型及其兼容入口。
pub mod abstract_entity;
/// 公开 `abstract_versioned_aggregate_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod abstract_versioned_aggregate_exception;
/// 公开 `aggregate_already_exists_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_already_exists_exception;
/// 公开 `aggregate_cache` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_cache;
/// 公开 `aggregate_deleted_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_deleted_exception;
/// 公开 `aggregate_no_cache` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_no_cache;
/// 公开 `aggregate_not_found_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_not_found_exception;
/// 公开 `aggregate_root` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_root;
/// 公开 `aggregate_root_id` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_root_id;
/// 公开 `aggregate_root_uuid` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_root_uuid;
/// 公开 `aggregate_version` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_version;
/// 公开 `aggregate_version_conflict_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_version_conflict_exception;
/// 公开 `aggregate_version_not_found_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod aggregate_version_not_found_exception;
/// 公开 `apply_event` 子模块，用于组织对应领域类型及其兼容入口。
pub mod apply_event;
/// 公开 `business_key` 子模块，用于组织对应领域类型及其兼容入口。
pub mod business_key;
/// 公开 `child_entity_locator` 子模块，用于组织对应领域类型及其兼容入口。
pub mod child_entity_locator;
/// 公开 `ddd4j_utils` 子模块，用于组织对应领域类型及其兼容入口。
pub mod ddd4j_utils;
/// 公开 `decryption_failed_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod decryption_failed_exception;
/// 公开 `domain_event` 子模块，用于组织对应领域类型及其兼容入口。
pub mod domain_event;
/// 公开 `domain_event_expected_entity_id_path` 子模块，用于组织对应领域类型及其兼容入口。
pub mod domain_event_expected_entity_id_path;
/// 公开 `domain_event_expected_entity_id_path_validator` 子模块，用于组织对应领域类型及其兼容入口。
pub mod domain_event_expected_entity_id_path_validator;
/// 公开 `duplicate_encryption_key_id_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod duplicate_encryption_key_id_exception;
/// 公开 `duplicate_entity_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod duplicate_entity_exception;
/// 公开 `encrypted_data` 子模块，用于组织对应领域类型及其兼容入口。
pub mod encrypted_data;
/// 公开 `encrypted_data_service` 子模块，用于组织对应领域类型及其兼容入口。
pub mod encrypted_data_service;
/// 公开 `encryption_key_id_unknown_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod encryption_key_id_unknown_exception;
/// 公开 `encryption_key_version_unknown_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod encryption_key_version_unknown_exception;
/// 公开 `entity` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity;
/// 公开 `entity_id` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity_id;
/// 公开 `entity_id_factory` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity_id_factory;
/// 公开 `entity_id_path` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity_id_path;
/// 公开 `entity_not_found_exception` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity_not_found_exception;
/// 公开 `entity_type` 子模块，用于组织对应领域类型及其兼容入口。
pub mod entity_type;
/// 公开 `event` 子模块，用于组织对应领域类型及其兼容入口。
pub mod event;
/// 公开 `event_id` 子模块，用于组织对应领域类型及其兼容入口。
pub mod event_id;
/// 公开 `event_type` 子模块，用于组织对应领域类型及其兼容入口。
pub mod event_type;
/// 公开 `exception_data` 子模块，用于组织对应领域类型及其兼容入口。
pub mod exception_data;
/// 公开 `exceptions` 子模块，用于组织对应领域类型及其兼容入口。
pub mod exceptions;
/// 公开 `expected_entity_id_path` 子模块，用于组织对应领域类型及其兼容入口。
pub mod expected_entity_id_path;
/// 公开 `expected_entity_id_path_validator` 子模块，用于组织对应领域类型及其兼容入口。
pub mod expected_entity_id_path_validator;
/// 公开 `has_entity_type_constant` 子模块，用于组织对应领域类型及其兼容入口。
pub mod has_entity_type_constant;
/// 公开 `has_entity_type_constant_validator` 子模块，用于组织对应领域类型及其兼容入口。
pub mod has_entity_type_constant_validator;
/// 公开 `integer_entity_id` 子模块，用于组织对应领域类型及其兼容入口。
pub mod integer_entity_id;
/// 公开 `jandex_entity_id_factory` 子模块，用于组织对应领域类型及其兼容入口。
pub mod jandex_entity_id_factory;
/// 公开 `method_executor` 子模块，用于组织对应领域类型及其兼容入口。
pub mod method_executor;
/// 公开 `repository` 子模块，用于组织对应领域类型及其兼容入口。
pub mod repository;
/// 公开 `string_based_entity_type` 子模块，用于组织对应领域类型及其兼容入口。
pub mod string_based_entity_type;

// Re-export everything
pub use abstract_aggregate_exception::AbstractAggregateException;
pub use abstract_aggregate_root::{AbstractAggregateRoot, ApplyEventHandler};
pub use abstract_entity::AbstractEntity;
pub use abstract_versioned_aggregate_exception::AbstractVersionedAggregateException;
pub use aggregate_already_exists_exception::AggregateAlreadyExistsException;
pub use aggregate_cache::AggregateCache;
pub use aggregate_deleted_exception::AggregateDeletedException;
pub use aggregate_no_cache::AggregateNoCache;
pub use aggregate_not_found_exception::AggregateNotFoundException;
pub use aggregate_root::AggregateRoot;
pub use aggregate_root_id::AggregateRootId;
pub use aggregate_root_uuid::AggregateRootUuid;
pub use aggregate_version::AggregateVersion;
pub use aggregate_version_conflict_exception::AggregateVersionConflictException;
pub use aggregate_version_not_found_exception::AggregateVersionNotFoundException;
pub use apply_event::ApplyEvent;
pub use business_key::BusinessKey;
pub use child_entity_locator::ChildEntityLocator;
pub use ddd4j_utils::Ddd4JUtils;
pub use decryption_failed_exception::DecryptionFailedException;
pub use domain_event::DomainEvent;
pub use domain_event_expected_entity_id_path::DomainEventExpectedEntityIdPath;
pub use domain_event_expected_entity_id_path_validator::DomainEventExpectedEntityIdPathValidator;
pub use duplicate_encryption_key_id_exception::DuplicateEncryptionKeyIdException;
pub use duplicate_entity_exception::DuplicateEntityException;
pub use encrypted_data::EncryptedData;
pub use encrypted_data_service::{EncryptedDataService, EncryptionError};
pub use encryption_key_id_unknown_exception::EncryptionKeyIdUnknownException;
pub use encryption_key_version_unknown_exception::EncryptionKeyVersionUnknownException;
pub use entity::Entity;
pub use entity_id::EntityId;
pub use entity_id_factory::EntityIdFactory;
pub use entity_id_path::{EntityIdPath, EntityIdPathError};
pub use entity_not_found_exception::EntityNotFoundException;
pub use entity_type::EntityType;
pub use event::Event;
pub use event_id::EventId;
pub use event_type::{EventType, EventTypeError};
pub use exception_data::ExceptionData;
pub use exceptions::AggregateError;
pub use expected_entity_id_path::ExpectedEntityIdPath;
pub use expected_entity_id_path_validator::ExpectedEntityIdPathValidator;
pub use has_entity_type_constant::HasEntityTypeConstant;
pub use has_entity_type_constant_validator::HasEntityTypeConstantValidator;
pub use integer_entity_id::IntegerEntityId;
pub use jandex_entity_id_factory::{
    EntityIdRegistration, EntityIdRegistryError, JandexEntityIdFactory,
};
pub use method_executor::{MethodExecutionError, MethodExecutor};
pub use repository::Repository;
pub use string_based_entity_type::{EntityTypeError, StringBasedEntityType};
