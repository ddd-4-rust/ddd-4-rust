//! DDD-4-Rust Test: Test utilities.
//!
//! 1:1 translation of `ddd-4-java-test`.
//!
//! Provides mock event store and test helpers.

use std::sync::Mutex;
use async_trait::async_trait;
use ddd_4_rust_esc::{CommonEvent, EventStore, EventStoreError, StreamEventsSlice, StreamId};

/// A simple in-memory event store for testing.
pub struct MemoryEventStore {
    events: Mutex<std::collections::HashMap<String, Vec<CommonEvent>>>,
}

impl MemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for MemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for MemoryEventStore {
    async fn read_stream_events_forward(
        &self,
        stream_id: &StreamId,
        start: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        let events_map = self.events.lock().unwrap();
        let events = events_map.get(stream_id.as_str()).cloned().unwrap_or_default();
        let start_idx = start.max(0) as usize;
        let end_idx = (start_idx + count as usize).min(events.len());
        let slice_events: Vec<CommonEvent> = events[start_idx..end_idx].to_vec();
        let last_event_number = events.last().map(|e| e.event_number);
        Ok(StreamEventsSlice {
            events: slice_events,
            next_event_number: if end_idx < events.len() {
                Some(end_idx as i64)
            } else {
                None
            },
            is_end_of_stream: end_idx >= events.len(),
            last_event_number,
        })
    }

    async fn append_to_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
        events: Vec<CommonEvent>,
    ) -> Result<(), EventStoreError> {
        let mut events_map = self.events.lock().unwrap();
        let stream = events_map.entry(stream_id.as_str().to_string()).or_default();
        let actual_version = if stream.is_empty() { -1 } else { stream.len() as i64 - 1 };
        if expected_version != -2 && expected_version != actual_version {
            return Err(EventStoreError::WrongExpectedVersion {
                expected: expected_version,
                actual: actual_version,
            });
        }
        stream.extend(events);
        Ok(())
    }

    async fn delete_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
    ) -> Result<(), EventStoreError> {
        let mut events_map = self.events.lock().unwrap();
        let stream = events_map.get(stream_id.as_str());
        let actual_version = match stream {
            Some(s) if !s.is_empty() => s.len() as i64 - 1,
            _ => -1,
        };
        if expected_version != actual_version {
            return Err(EventStoreError::WrongExpectedVersion {
                expected: expected_version,
                actual: actual_version,
            });
        }
        self.events.lock().unwrap().remove(stream_id.as_str());
        Ok(())
    }

    async fn read_all_events_forward(
        &self,
        _position: i64,
        _count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        Ok(StreamEventsSlice {
            events: vec![],
            next_event_number: None,
            is_end_of_stream: true,
            last_event_number: None,
        })
    }
}
