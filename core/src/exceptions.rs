//! Aggregate-related exceptions.
//!
//! 1:1 translation of the DDD exception classes.

use thiserror::Error;

/// Aggregate-related errors, matching the Java exception hierarchy.
///
/// Java exceptions:
/// - `AggregateNotFoundException`
/// - `AggregateVersionConflictException`
/// - `AggregateDeletedException`
/// - `AggregateAlreadyExistsException`
/// - `AggregateVersionNotFoundException`
/// - `AggregateVersionNotFoundException`
/// - `EntityNotFoundException`
/// - `DuplicateEntityException`
#[derive(Debug, Error)]
pub enum AggregateError {
    /// An aggregate with the given identifier was not found.
    ///
    /// Java: `AggregateNotFoundException`
    #[error("Aggregate not found: {entity_type} {entity_id}")]
    AggregateNotFound {
        /// Type of the aggregate.
        entity_type: String,
        /// Identifier of the aggregate.
        entity_id: String,
    },

    /// The expected version didn't match the actual version.
    ///
    /// Java: `AggregateVersionConflictException`
    #[error("Aggregate version conflict: expected {expected_version}, actual {actual_version}")]
    AggregateVersionConflict {
        /// Expected version.
        expected_version: u32,
        /// Actual version in the repository.
        actual_version: u32,
    },

    /// The aggregate with the given identifier was already deleted.
    ///
    /// Java: `AggregateDeletedException`
    #[error("Aggregate already deleted: {entity_type} {entity_id}")]
    AggregateDeleted {
        /// Type of the aggregate.
        entity_type: String,
        /// Identifier of the aggregate.
        entity_id: String,
    },

    /// The aggregate already exists when trying to create it.
    ///
    /// Java: `AggregateAlreadyExistsException`
    #[error("Aggregate already exists: {entity_type} {entity_id}")]
    AggregateAlreadyExists {
        /// Type of the aggregate.
        entity_type: String,
        /// Identifier of the aggregate.
        entity_id: String,
    },

    /// An aggregate with the requested version does not exist.
    ///
    /// Java: `AggregateVersionNotFoundException`
    #[error("Aggregate version not found: version {version}")]
    AggregateVersionNotFound {
        /// The version that was not found.
        version: u32,
    },

    /// An entity was not found.
    ///
    /// Java: `EntityNotFoundException`
    #[error("Entity not found: {entity_type} {entity_id}")]
    EntityNotFound {
        /// Type of the entity.
        entity_type: String,
        /// Identifier of the entity.
        entity_id: String,
    },

    /// A duplicate entity was detected.
    ///
    /// Java: `DuplicateEntityException`
    #[error("Duplicate entity: {entity_type} {entity_id}")]
    DuplicateEntity {
        /// Type of the entity.
        entity_type: String,
        /// Identifier of the entity.
        entity_id: String,
    },

    /// Inner/other error wrapped.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
