//! DDD-4-Rust ESC: Event Store Commons.
//!
//! 1:1 translation of `ddd-4-java-esc`.
//!
//! Provides event store abstractions and `EventStoreRepository`.

mod event_store;
mod event_store_repository;
mod aggregate_stream_id;

pub use event_store::{CommonEvent, EventStore, EventStoreError, StreamEventsSlice, StreamId};
pub use event_store_repository::EventStoreRepository;
pub use aggregate_stream_id::AggregateStreamId;
