//! Repository that supports CRUD operations for an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Repository`.

use crate::aggregate_root::AggregateRoot;
use crate::aggregate_root_id::AggregateRootId;
use crate::entity_type::EntityType;
use crate::exceptions::AggregateError;
use async_trait::async_trait;

/// Repository that supports CRUD operations for an aggregate.
///
/// Java: `Repository<ID extends AggregateRootId, T extends AggregateRoot<ID>>`
///
/// Methods (1:1):
/// - `getAggregateClass()` → `aggregate_type()` (returns EntityType instead of Class)
/// - `getAggregateType()` → `aggregate_type_name()`
/// - `create()` → `create()`
/// - `read(ID id)` → `read(id)`
/// - `read(ID id, int version)` → `read_at_version(id, version)`
/// - `update(T aggregate)` → `update(aggregate)`
/// - `update(T aggregate, String metaType, Object metaData)` → `update_with_meta(aggregate, meta_type, meta_data)`
/// - `add(T aggregate)` → `add(aggregate)`
/// - `add(T aggregate, String metaType, Object metaData)` → `add_with_meta(aggregate, meta_type, meta_data)`
/// - `delete(ID aggregateId, int expectedVersion)` → `delete(id, expected_version)`
#[async_trait]
pub trait Repository<ID: AggregateRootId + ?Sized, T: AggregateRoot<ID> + Send>: Send + Sync {
    /// Returns a unique name for the aggregate root type.
    ///
    /// Java: `getAggregateType() -> EntityType`
    fn aggregate_type(&self) -> &dyn EntityType;

    /// Factory method to create a new aggregate. Just creates a new instance without persisting.
    ///
    /// Java: `create() -> T`
    fn create(&self) -> T;

    /// Reads the latest version of an aggregate.
    ///
    /// Java: `read(ID id) -> T`
    async fn read(&self, id: &ID) -> Result<T, AggregateError>;

    /// Reads a given version of an aggregate.
    ///
    /// Java: `read(ID id, int version) -> T`
    async fn read_at_version(&self, id: &ID, version: u32) -> Result<T, AggregateError>;

    /// Saves the changes on an aggregate in the repository.
    ///
    /// Java: `update(T aggregate)`
    async fn update(&self, aggregate: &T) -> Result<(), AggregateError>;

    /// Saves the changes on an aggregate with metadata.
    ///
    /// Java: `update(T aggregate, String metaType, Object metaData)`
    async fn update_with_meta(
        &self,
        aggregate: &T,
        meta_type: Option<&str>,
        meta_data: Option<&[u8]>,
    ) -> Result<(), AggregateError>;

    /// Adds a new aggregate to the repository.
    ///
    /// Java: `add(T aggregate)`
    async fn add(&self, aggregate: &T) -> Result<(), AggregateError>;

    /// Adds a new aggregate to the repository with metadata.
    ///
    /// Java: `add(T aggregate, String metaType, Object metaData)`
    async fn add_with_meta(
        &self,
        aggregate: &T,
        meta_type: Option<&str>,
        meta_data: Option<&[u8]>,
    ) -> Result<(), AggregateError>;

    /// Deletes an aggregate from the repository.
    ///
    /// Java: `delete(ID aggregateId, int expectedVersion)`
    async fn delete(&self, id: &ID, expected_version: u32) -> Result<(), AggregateError>;
}
