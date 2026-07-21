//! DDD-4-Rust Jackson: serde-based serialization for DDD building blocks.
//!
//! 1:1 translation of `ddd-4-java-jackson`.
//!
//! Provides `AbstractEvent` and `AbstractDomainEvent` base structs with serde support.

mod abstract_event;
mod abstract_domain_event;

pub use abstract_event::AbstractEvent;
pub use abstract_domain_event::AbstractDomainEvent;
