//! Base implementation for domain events with JSON-B/serde serialization.
//!
//! 1:1 translation of `org.fuin.ddd4j.jsonb.AbstractDomainEvent`.

use chrono::{DateTime, Utc};
use ddd_4_rust_core::{AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType};
use serde::{Deserialize, Serialize};

/// Base struct for domain events with serde serialization support (JSON-B equivalent).
///
/// Java: `AbstractDomainEvent<ID extends EntityId> extends AbstractEvent implements DomainEvent<ID>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractDomainEvent {
    event_id: EventId,
    event_type: EventType,
    event_timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    causation_id: Option<EventId>,
    entity_id_path: EntityIdPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_version: Option<AggregateVersion>,
}

impl AbstractDomainEvent {
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
    ) -> Self {
        Self {
            event_id, event_type, event_timestamp,
            correlation_id, causation_id,
            entity_id_path, aggregate_version,
        }
    }

    pub fn new_now(
        event_type: EventType,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
    ) -> Self {
        Self {
            event_id: EventId::new(),
            event_type,
            event_timestamp: Utc::now(),
            correlation_id: None,
            causation_id: None,
            entity_id_path,
            aggregate_version,
        }
    }

    pub fn event_id(&self) -> &EventId { &self.event_id }
    pub fn event_type(&self) -> &EventType { &self.event_type }
    pub fn event_timestamp(&self) -> &DateTime<Utc> { &self.event_timestamp }
    pub fn correlation_id(&self) -> Option<&EventId> { self.correlation_id.as_ref() }
    pub fn causation_id(&self) -> Option<&EventId> { self.causation_id.as_ref() }
    pub fn entity_id_path(&self) -> &EntityIdPath { &self.entity_id_path }
    pub fn aggregate_version(&self) -> Option<&AggregateVersion> { self.aggregate_version.as_ref() }
}

impl Event for AbstractDomainEvent {
    fn event_id(&self) -> &EventId { &self.event_id }
    fn event_type(&self) -> &EventType { &self.event_type }
    fn event_timestamp(&self) -> &DateTime<Utc> { &self.event_timestamp }
    fn correlation_id(&self) -> Option<&EventId> { self.correlation_id.as_ref() }
    fn causation_id(&self) -> Option<&EventId> { self.causation_id.as_ref() }
}

impl DomainEvent<dyn EntityId> for AbstractDomainEvent {
    fn entity_id_path(&self) -> &EntityIdPath { &self.entity_id_path }
    fn entity_id(&self) -> &dyn EntityId { self.entity_id_path.last().as_ref() }
    fn aggregate_version(&self) -> Option<&AggregateVersion> { self.aggregate_version.as_ref() }
}
