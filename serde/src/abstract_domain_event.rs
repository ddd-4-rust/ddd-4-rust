//! Base implementation for domain events with Jackson/serde serialization.
//!
//! 1:1 translation of `org.fuin.ddd4j.jackson.AbstractDomainEvent`.

use chrono::{DateTime, Utc};
use ddd_4_rust_core::{AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType};
use serde::{Deserialize, Serialize};

/// Base struct for domain events with serde serialization support.
///
/// Users should embed this in their domain event structs.
///
/// Java: `AbstractDomainEvent<ID extends EntityId> extends AbstractEvent implements DomainEvent<ID>`
///
/// # Example
///
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// struct PersonCreatedEvent {
///     #[serde(flatten)]
///     base: AbstractDomainEvent,
///     name: String,
/// }
/// ```
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
    /// Creates a new abstract domain event.
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
            event_id,
            event_type,
            event_timestamp,
            correlation_id,
            causation_id,
            entity_id_path,
            aggregate_version,
        }
    }

    /// Creates a new abstract domain event with defaults.
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

    /// Returns the event ID.
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    /// Returns the event type.
    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    /// Returns the event timestamp.
    pub fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.event_timestamp
    }

    /// Returns the correlation ID.
    pub fn correlation_id(&self) -> Option<&EventId> {
        self.correlation_id.as_ref()
    }

    /// Returns the causation ID.
    pub fn causation_id(&self) -> Option<&EventId> {
        self.causation_id.as_ref()
    }

    /// Returns the entity ID path.
    pub fn entity_id_path(&self) -> &EntityIdPath {
        &self.entity_id_path
    }

    /// Returns the aggregate version.
    pub fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.aggregate_version.as_ref()
    }
}

impl Event for AbstractDomainEvent {
    fn event_id(&self) -> &EventId {
        &self.event_id
    }

    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.event_timestamp
    }

    fn correlation_id(&self) -> Option<&EventId> {
        self.correlation_id.as_ref()
    }

    fn causation_id(&self) -> Option<&EventId> {
        self.causation_id.as_ref()
    }
}

// Note: DomainEvent<EntityId> requires entity_id() to return &ID (the last ID in the path).
// Since AbstractDomainEvent doesn't know the concrete EntityId type statically,
// this implementation returns the last ID as &dyn EntityId.
impl DomainEvent<dyn EntityId> for AbstractDomainEvent {
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.entity_id_path
    }

    fn entity_id(&self) -> &dyn EntityId {
        // Return the last entity ID in the path
        self.entity_id_path.last().as_ref()
    }

    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.aggregate_version.as_ref()
    }
}
