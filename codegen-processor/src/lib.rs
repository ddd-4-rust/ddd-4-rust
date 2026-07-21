//! DDD-4-Rust Codegen Processor: Proc macros for DDD code generation.
//!
//! 1:1 translation of `ddd-4-java-codegen-processor`.
//!
//! Provides attribute macros:
//! - `#[apply_event]` — generates `try_apply_event` dispatch match arms
//! - `#[child_locator]` — registers child entity lookup (stub for now)

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ImplItem, ItemImpl};

/// Attribute macro that generates the `try_apply_event` method
/// for all methods in the impl block that handle specific event types.
///
/// Java: `@ApplyEvent` annotation
///
/// # Usage
///
/// ```ignore
/// #[apply_event]
/// impl Person {
///     fn apply_person_created(&mut self, event: &PersonCreatedEvent) { ... }
///     fn apply_person_deleted(&mut self, event: &PersonDeletedEvent) { ... }
/// }
/// ```
///
/// Generates:
/// ```ignore
/// impl ApplyEventHandler for Person {
///     fn try_apply_event(&mut self, event: &dyn DomainEvent<dyn EntityId>) -> bool {
///         match event.event_type().as_str() {
///             PersonCreatedEvent::EVENT_TYPE => {
///                 if let Some(e) = (event as &dyn Any).downcast_ref::<PersonCreatedEvent>() {
///                     self.apply_person_created(e);
///                     return true;
///                 }
///                 false
///             }
///             PersonDeletedEvent::EVENT_TYPE => { ... }
///             _ => false,
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn apply_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let self_ty = &input.self_ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut event_handlers = Vec::new();

    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;

            // Check method signature: fn xxx(&mut self, event: &EventType)
            if method.sig.inputs.len() >= 2 {
                // Get the second argument (first is &mut self)
                if let syn::FnArg::Typed(pat_type) = &method.sig.inputs[1] {
                    if let syn::Type::Reference(type_ref) = &*pat_type.ty {
                        if let syn::Type::Path(type_path) = &*type_ref.elem {
                            let event_type = &type_path.path;
                            let event_type_name = quote!(#event_type).to_string();
                            // Derive the EVENT_TYPE constant name
                            let const_name = format_ident!("{}", event_type_name);

                            event_handlers.push((method_name.clone(), event_type.clone(), const_name));
                        }
                    }
                }
            }
        }
    }

    // Generate match arms
    let match_arms = event_handlers.iter().map(|(method_name, event_type, const_name)| {
        quote! {
            #const_name::EVENT_TYPE => {
                if let Some(e) = (event as &dyn ::std::any::Any).downcast_ref::<#event_type>() {
                    self.#method_name(e);
                    return true;
                }
                false
            }
        }
    });

    let expanded = quote! {
        #input

        impl #impl_generics ddd_4_rust_core::abstract_aggregate_root::ApplyEventHandler for #self_ty #ty_generics #where_clause {
            fn try_apply_event(&mut self, event: &dyn ddd_4_rust_core::domain_event::DomainEvent<dyn ddd_4_rust_core::entity_id::EntityId>) -> bool {
                match event.event_type().as_str() {
                    #(#match_arms)*
                    _ => false,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for child entity locator methods.
///
/// Java: `@ChildEntityLocator` annotation
///
/// # Usage
///
/// ```ignore
/// #[child_locator]
/// impl MyAggregate {
///     fn find_child(&self, id: &ChildId) -> Option<&ChildEntity> { ... }
/// }
/// ```
#[proc_macro_attribute]
pub fn child_locator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For now, pass through. Full implementation will:
    // 1. Discover methods annotated with child_locator
    // 2. Register them for aggregate root traversal
    item
}
