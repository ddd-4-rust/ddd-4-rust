//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! DDD-4-Rust ESC: Event Store Commons.
//!
//! 1:1 translation of `ddd-4-java-esc`.
//!
//! Provides event store abstractions and `EventStoreRepository`.

#![expect(
    clippy::missing_errors_doc,
    reason = "repository errors preserve the Java exception mapping and are documented on AggregateError"
)]

mod aggregate_stream_id;
mod event_store;
mod event_store_repository;
mod package_info;

pub use aggregate_stream_id::AggregateStreamId;
pub use event_store::{CommonEvent, EventStore, EventStoreError, StreamEventsSlice, StreamId};
pub use event_store_repository::EventStoreRepository;
pub use package_info::JAVA_PACKAGE as JAVA_PACKAGE_NAME;
