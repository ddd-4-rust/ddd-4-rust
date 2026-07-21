//! Identifies an aggregate within all aggregates of the same type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateRootId`.

use crate::entity_id::EntityId;

/// Identifies an aggregate within all aggregates of the same type.
///
/// Java: `AggregateRootId extends EntityId`
pub trait AggregateRootId: EntityId {}
