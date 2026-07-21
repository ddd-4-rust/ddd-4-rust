//! Utility methods and constants.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Ddd4JUtils`.

use crate::event_type::EventType;

/// Utility methods and constants for DDD operations.
///
/// Java: `Ddd4JUtils`
pub struct Ddd4JUtils;

impl Ddd4JUtils {
    /// Prefix for unique short identifiers.
    ///
    /// Java: `SHORT_ID_PREFIX = "DDD4J"`
    pub const SHORT_ID_PREFIX: &'static str = "DDD4J";

    /// Creates an Adler32 checksum based on event type names.
    ///
    /// Java: `calculateChecksum(Collection<EventType> eventTypes) -> long`
    pub fn calculate_checksum(event_types: &[EventType]) -> u32 {
        let mut adler = adler32::Adler32::new();
        for et in event_types {
            adler.update(et.as_str().as_bytes());
        }
        adler.finish()
    }
}

// Minimal Adler32 implementation (std-only, no external crate)
mod adler32 {
    const MOD_ADLER: u32 = 65521;

    pub struct Adler32 {
        a: u32,
        b: u32,
    }

    impl Adler32 {
        pub fn new() -> Self {
            Self { a: 1, b: 0 }
        }

        pub fn update(&mut self, data: &[u8]) {
            for &byte in data {
                self.a = (self.a + byte as u32) % MOD_ADLER;
                self.b = (self.b + self.a) % MOD_ADLER;
            }
        }

        pub fn finish(&self) -> u32 {
            (self.b << 16) | self.a
        }
    }
}
