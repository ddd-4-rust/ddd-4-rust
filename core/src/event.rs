//! Something that happened in the system.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Event`.

use crate::event_id::EventId;
use crate::event_type::EventType;
use chrono::{DateTime, Utc};
use std::any::Any;

/// Something that happened in the system.
///
/// Java: `Event extends Serializable`
///
/// Methods:
/// - `getEventId()` → `event_id()`
/// - `getEventType()` → `event_type()`
/// - `getEventTimestamp()` → `event_timestamp()`
/// - `getCorrelationId()` → `correlation_id()`
/// - `getCausationId()` → `causation_id()`
pub trait Event: Any + Send + Sync {
    /// Returns the identifier of the event.
    ///
    /// Java: `@NotNull getEventId()`
    fn event_id(&self) -> &EventId;

    /// Returns the type of the event (What happened).
    ///
    /// Java: `@NotNull getEventType()`
    fn event_type(&self) -> &EventType;

    /// Date, time and time zone the event was created.
    ///
    /// Java: `@NotNull getEventTimestamp()`
    fn event_timestamp(&self) -> &DateTime<Utc>;

    /// Correlation identifier.
    ///
    /// Java: `@Nullable getCorrelationId()`
    fn correlation_id(&self) -> Option<&EventId>;

    /// Causation identifier.
    ///
    /// Java: `@Nullable getCausationId()`
    fn causation_id(&self) -> Option<&EventId>;
}

/// Helper to downcast a dyn Event to a concrete type.
#[allow(dead_code)]
pub fn downcast_event<T: Event + 'static>(event: &dyn Event) -> Option<&T> {
    (event as &dyn Any).downcast_ref::<T>()
}
