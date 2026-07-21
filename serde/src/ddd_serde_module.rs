//! Serde module registration for DDD types.
//!
//! 1:1 translation of `Ddd4JacksonModule`.

/// Serde module for DDD types.
///
/// In Java, `Ddd4JacksonModule` registers custom Jackson serializers/deserializers.
/// In Rust with serde, custom adapters are used via `#[serde(with = "...")]` or
/// manual `Serialize`/`Deserialize` implementations.
///
/// Java: `Ddd4JacksonModule extends Module`
pub struct DddSerdeModule;

impl DddSerdeModule {
    /// Creates a new DDD serde module.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DddSerdeModule {
    fn default() -> Self {
        Self::new()
    }
}
