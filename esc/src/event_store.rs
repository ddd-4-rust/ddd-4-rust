//! Event store abstraction.
//!
//! 1:1 translation of `org.fuin.esc.api.CommonEvent`, `StreamId`, `EventStore`, etc.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique stream identifier.
///
/// Java: `StreamId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StreamId(String);

impl StreamId {
    /// Creates a new stream ID.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Returns the stream name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A common event as stored in the event store.
///
/// Java: `CommonEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonEvent {
    /// Unique event identifier.
    pub event_id: Uuid,
    /// Event type name.
    pub event_type: String,
    /// Event data as raw bytes/JSON.
    pub data: Vec<u8>,
    /// Optional metadata.
    pub metadata: Option<Vec<u8>>,
    /// Event creation timestamp.
    pub created: DateTime<Utc>,
    /// Stream position / event number.
    pub event_number: i64,
}

/// A slice of events read from a stream.
///
/// Java: `StreamEventsSlice`
#[derive(Debug, Clone)]
pub struct StreamEventsSlice {
    /// The events in this slice.
    pub events: Vec<CommonEvent>,
    /// The next event number to read from, or None if end of stream.
    pub next_event_number: Option<i64>,
    /// Whether this is the end of the stream.
    pub is_end_of_stream: bool,
    /// The last event number in the stream.
    pub last_event_number: Option<i64>,
}

/// Event store abstraction.
///
/// Java: `EventStore`
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Reads events forward from a stream.
    async fn read_stream_events_forward(
        &self,
        stream_id: &StreamId,
        start: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError>;

    /// Appends events to a stream.
    async fn append_to_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
        events: Vec<CommonEvent>,
    ) -> Result<(), EventStoreError>;

    /// Deletes a stream.
    async fn delete_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
    ) -> Result<(), EventStoreError>;

    /// Reads all events forward (for projections).
    async fn read_all_events_forward(
        &self,
        position: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError>;
}

/// Event store errors.
#[derive(Debug, thiserror::Error)]
pub enum EventStoreError {
    #[error("Stream not found: {0}")]
    StreamNotFound(String),
    #[error("Wrong expected version: expected {expected}, actual {actual}")]
    WrongExpectedVersion { expected: i64, actual: i64 },
    #[error("Stream deleted: {0}")]
    StreamDeleted(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("{0}")]
    Other(String),
}
