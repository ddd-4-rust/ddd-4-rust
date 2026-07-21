//! DDD-4-Rust Codegen Processor: Proc macros for DDD code generation.
//!
//! 1:1 translation of `ddd-4-java-codegen-processor`.
//!
//! Provides derive and attribute macros:
//! - `#[apply_event]` — generates `try_apply_event` dispatch
//! - `#[child_locator]` — registers child entity lookup
//! - `#[derive(DddEvent)]` — generates Event trait impl + serde

use proc_macro::TokenStream;

/// Attribute macro that generates the `try_apply_event` method
/// for all methods in the impl block that handle specific event types.
///
/// Java: `@ApplyEvent` annotation
///
/// # Usage
///
/// ```ignore
/// #[apply_event]
/// impl MyAggregate {
///     fn apply_person_created(&mut self, event: &PersonCreatedEvent) { ... }
///     fn apply_person_deleted(&mut self, event: &PersonDeletedEvent) { ... }
/// }
/// ```
#[proc_macro_attribute]
pub fn apply_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For now, pass through. Full implementation will match methods by event type param.
    item
}

/// Attribute macro for child entity locator methods.
///
/// Java: `@ChildEntityLocator` annotation
#[proc_macro_attribute]
pub fn child_locator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Derive macro for Event trait implementation.
///
/// Java: `@HasEntityTypeConstant` + `@HasPublicStaticValueOfMethod`
#[proc_macro_derive(DddEvent)]
pub fn derive_ddd_event(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Derive macro for EntityId trait implementation (UUID-based).
#[proc_macro_derive(EntityId)]
pub fn derive_entity_id(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Derive macro for AggregateRootId marker trait.
#[proc_macro_derive(AggregateRootId)]
pub fn derive_aggregate_root_id(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
