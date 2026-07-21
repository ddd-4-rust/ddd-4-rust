//! Human-readable business (natural) key marker trait.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.BusinessKey`.

/// Human-readable business (natural) key often used in documents or shown in the user interface.
///
/// Java: `BusinessKey extends Serializable`
///
/// Marker trait for value objects that represent a business/natural key.
pub trait BusinessKey: Send + Sync + 'static {}
