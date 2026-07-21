//! DDD-4-Rust JSON-B: serde-based serialization for DDD building blocks.
//!
//! 1:1 translation of `ddd-4-java-jsonb`.
//!
//! Provides `AbstractEvent` and `AbstractDomainEvent` base structs with serde support,
//! equivalent to the JSON-B (Jakarta JSON Binding) module in Java.

mod abstract_event;
mod abstract_domain_event;

pub use abstract_event::AbstractEvent;
pub use abstract_domain_event::AbstractDomainEvent;
