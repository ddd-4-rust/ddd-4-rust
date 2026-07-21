//! DDD-4-Rust Codegen API: Traits for code generation.
//!
//! 1:1 translation of `ddd-4-java-codegen-api`.
//!
//! Defines traits and types used by the codegen processor.

/// Placeholder for code generation API.
/// In Java, this provides annotation types like `@HasEntityTypeConstant`,
/// `@HasPublicStaticValueOfMethod`, etc. In Rust, these become derive macros
/// in the `ddd-4-rust-codegen-processor` crate.
pub struct CodegenMarker;
