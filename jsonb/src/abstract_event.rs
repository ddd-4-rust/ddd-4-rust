//! Base implementation for events with JSON-B/serde serialization.
//!
//! 1:1 translation of `org.fuin.ddd4j.jsonb.AbstractEvent`.

use chrono::{DateTime, Utc};
use ddd_4_rust_core::{Event, EventId, EventType};
use serde::{Deserialize, Serialize};

/// Base struct for events with serde serialization support (JSON-B equivalent).
///
/// Java: `AbstractEvent extends AbstractEvent implements Event`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractEvent {
    event_id: EventId,
    event_type: EventType,
    event_timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    causation_id: Option<EventId>,
}

impl AbstractEvent {
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_timestamp,
            correlation_id,
            causation_id,
        }
    }

    pub fn new_now(event_type: EventType) -> Self {
        Self {
            event_id: EventId::new(),
            event_type,
            event_timestamp: Utc::now(),
            correlation_id: None,
            causation_id: None,
        }
    }

    pub fn event_id(&self) -> &EventId { &self.event_id }
    pub fn event_type(&self) -> &EventType { &self.event_type }
    pub fn event_timestamp(&self) -> &DateTime<Utc> { &self.event_timestamp }
    pub fn correlation_id(&self) -> Option<&EventId> { self.correlation_id.as_ref() }
    pub fn causation_id(&self) -> Option<&EventId> { self.causation_id.as_ref() }
}

impl Event for AbstractEvent {
    fn event_id(&self) -> &EventId { &self.event_id }
    fn event_type(&self) -> &EventType { &self.event_type }
    fn event_timestamp(&self) -> &DateTime<Utc> { &self.event_timestamp }
    fn correlation_id(&self) -> Option<&EventId> { self.correlation_id.as_ref() }
    fn causation_id(&self) -> Option<&EventId> { self.causation_id.as_ref() }
}
