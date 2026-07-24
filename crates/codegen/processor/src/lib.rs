//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! Procedural macros replacing the frozen Java annotation processor.

use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use syn::{
    Data, DeriveInput, Fields, FnArg, ImplItem, ItemImpl, ReturnType, Type, parse_macro_input,
};

#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod aggregate_root_uuid_vo_template;
mod ddd4j_code_gen_utils;
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod event_vo_template;
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod integer_entity_id_vo_template;
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod string_vo_template;
mod value_object_processor;
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod value_object_template;

/// Converts a `syn::Error` into a compile-error token stream.
fn compile_error(error: syn::Error) -> proc_macro2::TokenStream {
    error.into_compile_error()
}

/// Rejects non-empty attribute token streams for attribute macros.
fn require_empty_attr(is_empty: bool, message: &str) -> Result<(), syn::Error> {
    if !is_empty {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), message));
    }
    Ok(())
}

/// Validated apply-event handler `(method_ident, event_type_path)`.
type ApplyEventHandlerRef<'a> = (&'a syn::Ident, &'a syn::Path);

/// Validates `#[apply_event]` impl contents and returns dispatch targets.
fn validate_apply_event_impl(
    input: &ItemImpl,
) -> Result<Vec<ApplyEventHandlerRef<'_>>, syn::Error> {
    let mut handlers = Vec::new();
    let mut event_types = BTreeSet::new();
    for item in &input.items {
        let ImplItem::Fn(method) = item else {
            continue;
        };
        if method.sig.inputs.len() != 2
            || !matches!(method.sig.inputs.first(), Some(FnArg::Receiver(receiver)) if matches!(&receiver.kind, syn::ReceiverKind::Reference(_, _, Some(_))))
        {
            return Err(syn::Error::new_spanned(
                &method.sig,
                "event handler must have the form fn handler(&mut self, event: &Event) -> Result<(), AggregateError>",
            ));
        }
        if !matches!(method.sig.output, ReturnType::Type(_, ref output) if matches!(&**output, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Result")))
        {
            return Err(syn::Error::new_spanned(
                &method.sig.output,
                "event handler must return Result",
            ));
        }
        let Some(FnArg::Typed(argument)) = method.sig.inputs.iter().nth(1) else {
            return Err(syn::Error::new_spanned(
                &method.sig,
                "event handler must have the form fn handler(&mut self, event: &Event) -> Result<(), AggregateError>",
            ));
        };
        let Type::Reference(reference) = &*argument.ty else {
            return Err(syn::Error::new_spanned(
                &argument.ty,
                "event handler argument must be a shared event reference",
            ));
        };
        let Type::Path(event_type) = &*reference.elem else {
            return Err(syn::Error::new_spanned(
                &reference.elem,
                "event handler argument must name a concrete event type",
            ));
        };
        let event_key = quote!(#event_type).to_string();
        if !event_types.insert(event_key) {
            return Err(syn::Error::new_spanned(
                &argument.ty,
                "duplicate event handler for this event type",
            ));
        }
        handlers.push((&method.sig.ident, &event_type.path));
    }
    if handlers.is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "#[apply_event] requires at least one event handler",
        ));
    }
    Ok(handlers)
}

/// Validates `#[child_locator]` impl contents.
fn validate_child_locator_impl(input: &ItemImpl) -> Result<(), syn::Error> {
    let methods: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .collect();
    if methods.is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "#[child_locator] requires at least one locator method",
        ));
    }
    for method in methods {
        let valid_receiver = matches!(method.sig.inputs.first(), Some(FnArg::Receiver(receiver)) if matches!(&receiver.kind, syn::ReceiverKind::Reference(_, _, None)));
        let valid_result = matches!(method.sig.output, ReturnType::Type(_, ref output) if matches!(&**output, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Option")));
        if method.sig.inputs.len() != 2 || !valid_receiver || !valid_result {
            return Err(syn::Error::new_spanned(
                &method.sig,
                "child locator must have the form fn locator(&self, id: &Id) -> Option<&Entity>",
            ));
        }
    }
    Ok(())
}

/// Builds `ApplyEventHandler` tokens after attribute and impl validation.
fn expand_apply_event(
    attr_is_empty: bool,
    input: ItemImpl,
) -> proc_macro2::TokenStream {
    if let Err(error) =
        require_empty_attr(attr_is_empty, "#[apply_event] does not accept arguments")
    {
        return compile_error(error);
    }
    let handlers = match validate_apply_event_impl(&input) {
        Ok(handlers) => handlers,
        Err(error) => return compile_error(error),
    };
    let self_ty = &input.self_ty;
    let (impl_generics, _, where_clause) = input.generics.split_for_impl();
    let dispatch = handlers.iter().map(|(method, event_type)| quote! {
        if event.event_type().as_str() == stringify!(#event_type) {
            if let Some(typed_event) = (event as &dyn ::std::any::Any).downcast_ref::<#event_type>() {
                self.#method(typed_event)?;
                return ::core::result::Result::Ok(true);
            }
            return ::core::result::Result::Ok(false);
        }
    });
    quote! {
        #input
        impl #impl_generics ddd_4_rust_core::ApplyEventHandler for #self_ty #where_clause {
            /// 执行 `try_apply_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
            /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
            fn try_apply_event(
                &mut self,
                event: &dyn ddd_4_rust_core::DomainEvent<dyn ddd_4_rust_core::EntityId>,
            ) -> ::core::result::Result<bool, ddd_4_rust_core::AggregateError> {
                #(#dispatch)*
                ::core::result::Result::Ok(false)
            }
        }
    }
}

/// Generates `ApplyEventHandler` dispatch for checked `Result` event handlers.
#[proc_macro_attribute]
/// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn apply_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    expand_apply_event(attr.is_empty(), input).into()
}

/// Builds child-locator passthrough tokens after attribute and impl validation.
fn expand_child_locator(
    attr_is_empty: bool,
    input: ItemImpl,
) -> proc_macro2::TokenStream {
    if let Err(error) =
        require_empty_attr(attr_is_empty, "#[child_locator] does not accept arguments")
    {
        return compile_error(error);
    }
    if let Err(error) = validate_child_locator_impl(&input) {
        return compile_error(error);
    }
    quote!(#input)
}

/// Validates an impl block containing child-entity locator methods.
#[proc_macro_attribute]
/// 执行 `child_locator` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn child_locator(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    expand_child_locator(attr.is_empty(), input).into()
}

/// Requires a single-field struct for value-object derives.
fn single_field(input: &DeriveInput) -> Result<&syn::Field, syn::Error> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "value-object derives require a struct",
        ));
    };
    let mut fields = data.fields.iter();
    let field = fields.next().ok_or_else(|| {
        syn::Error::new_spanned(&data.fields, "value object requires exactly one field")
    })?;
    if fields.next().is_some() {
        return Err(syn::Error::new_spanned(
            &data.fields,
            "value object requires exactly one field",
        ));
    }
    Ok(field)
}

/// Builds a field access expression for named or tuple structs.
fn field_access(fields: &Fields) -> Result<proc_macro2::TokenStream, syn::Error> {
    match fields {
        Fields::Named(named) => {
            let Some(field) = named.named.first() else {
                return Err(syn::Error::new_spanned(
                    fields,
                    "value object requires one named field",
                ));
            };
            let Some(ident) = field.ident.as_ref() else {
                return Err(syn::Error::new_spanned(
                    fields,
                    "value object requires one named field",
                ));
            };
            Ok(quote!(self.#ident))
        }
        Fields::Unnamed(_) => Ok(quote!(self.0)),
        Fields::Unit => Err(syn::Error::new_spanned(
            fields,
            "value object cannot be a unit struct",
        )),
    }
}

/// Resolves the single struct field access used by string value-object expansion.
fn string_value_object_access(
    input: &DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let field = single_field(input)?;
    match field.ident.as_ref() {
        Some(ident) => Ok(quote!(self.#ident)),
        None => Ok(quote!(self.0)),
    }
}

/// Expands `StringValueObject` for unit tests and the derive macro.
fn expand_string_value_object(input: DeriveInput) -> proc_macro2::TokenStream {
    let access = match string_value_object_access(&input) {
        Ok(access) => access,
        Err(error) => return compile_error(error),
    };
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "创建生成的字符串值对象，并把输入转换为对象持有的稳定字符串。"] pub fn new(value: impl Into<::std::string::String>) -> Self { Self(value.into()) } #[doc = "以借用形式返回生成对象保存的字符串，不复制数据也不修改对象状态。"] pub fn as_str(&self) -> &str { &#access } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { formatter.write_str(&#access) } } }
}

/// Expands `IntegerEntityIdValueObject` for unit tests and the derive macro.
fn expand_integer_entity_id_value_object(input: DeriveInput) -> proc_macro2::TokenStream {
    if let Err(error) = single_field(&input) {
        return compile_error(error);
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "根据给定整数创建生成的实体标识，保持值对象的强类型边界。"] pub const fn new(value: i32) -> Self { Self(value) } #[doc = "返回实体标识保存的整数值，不修改当前对象。"] pub const fn as_i32(&self) -> i32 { self.0 } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(formatter, "{}", self.0) } } }
}

/// Expands `AggregateRootUuidValueObject` for unit tests and the derive macro.
fn expand_aggregate_root_uuid_value_object(input: DeriveInput) -> proc_macro2::TokenStream {
    if let Err(error) = single_field(&input) {
        return compile_error(error);
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "生成随机 UUID 并创建新的聚合根标识，适用于新聚合的身份初始化。"] pub fn new() -> Self { Self(::uuid::Uuid::new_v4()) } #[doc = "根据已有 UUID 创建聚合根标识，用于反序列化和历史数据恢复。"] pub const fn from_uuid(value: ::uuid::Uuid) -> Self { Self(value) } #[doc = "以借用形式返回聚合根标识中的 UUID，不复制数据也不改变状态。"] pub const fn as_uuid(&self) -> &::uuid::Uuid { &self.0 } } impl #impl_generics ::std::default::Default for #name #ty_generics #where_clause { fn default() -> Self { Self::new() } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(formatter, "{}", self.0) } } }
}

/// Generates a checked string-value-object API for a one-field struct.
#[proc_macro_derive(StringValueObject)]
/// 执行 `string_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn string_value_object(item: TokenStream) -> TokenStream {
    expand_string_value_object(parse_macro_input!(item as DeriveInput)).into()
}

/// Generates an integer entity-ID value-object API for a one-field tuple struct.
#[proc_macro_derive(IntegerEntityIdValueObject)]
/// 执行 `integer_entity_id_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn integer_entity_id_value_object(item: TokenStream) -> TokenStream {
    expand_integer_entity_id_value_object(parse_macro_input!(item as DeriveInput)).into()
}

/// Generates a UUID aggregate-root-ID value-object API for a one-field tuple struct.
#[proc_macro_derive(AggregateRootUuidValueObject)]
/// 执行 `aggregate_root_uuid_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn aggregate_root_uuid_value_object(item: TokenStream) -> TokenStream {
    expand_aggregate_root_uuid_value_object(parse_macro_input!(item as DeriveInput)).into()
}

/// Generates the stable event type name used by dispatch code.
#[proc_macro_derive(EventValueObject)]
/// 执行 `event_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn event_value_object(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "保存由类型名称生成的稳定事件类型标识，供序列化和事件分派共同使用。"] pub const EVENT_TYPE: &'static str = stringify!(#name); } }.into()
}

#[cfg(test)]
mod internal_coverage {
    use super::{
        aggregate_root_uuid_vo_template::AggregateRootUuidVoTemplate, compile_error,
        ddd4j_code_gen_utils::Ddd4jCodeGenUtils,
        event_vo_template::{Clasz, EventVoTemplate, Field},
        expand_aggregate_root_uuid_value_object, expand_apply_event, expand_child_locator,
        expand_integer_entity_id_value_object, expand_string_value_object, field_access,
        integer_entity_id_vo_template::IntegerEntityIdVoTemplate, require_empty_attr,
        single_field, string_vo_template::StringVoTemplate, string_value_object_access,
        validate_apply_event_impl, validate_child_locator_impl,
        value_object_processor::{TargetType, ValueObjectProcessor},
        value_object_template::ValueObjectTemplate,
    };
    use quote::quote;
    use syn::{Data, DeriveInput, Fields, ImplItem, ItemImpl, parse_quote};

    /// Test helper that unwraps struct fields and panics for non-structs.
    fn struct_fields_for_test(input: &DeriveInput) -> &Fields {
        match &input.data {
            Data::Struct(data) => &data.fields,
            Data::Enum(_) | Data::Union(_) => panic!("expected struct"),
        }
    }

    /// Rewrites the second method input to `self` when the first impl item is a fn.
    fn force_second_input_self(item: &mut ItemImpl) {
        let Some(ImplItem::Fn(method)) = item.items.get_mut(0) else {
            return;
        };
        method.sig.inputs[1] = parse_quote! { self };
    }

    /// 覆盖各模板 `derive_name` 与代码生成工具函数。
    #[test]
    fn template_derive_names_and_utils_are_stable() {
        assert_eq!(StringVoTemplate::derive_name(), "StringValueObject");
        assert_eq!(
            IntegerEntityIdVoTemplate::derive_name(),
            "IntegerEntityIdValueObject"
        );
        assert_eq!(
            AggregateRootUuidVoTemplate::derive_name(),
            "AggregateRootUuidValueObject"
        );
        assert_eq!(
            ValueObjectProcessor::derive_name(TargetType::String),
            "StringValueObject"
        );
        assert_eq!(
            ValueObjectProcessor::derive_name(TargetType::IntegerEntityId),
            "IntegerEntityIdValueObject"
        );
        assert_eq!(
            ValueObjectProcessor::derive_name(TargetType::AggregateRootUuid),
            "AggregateRootUuidValueObject"
        );
        assert_eq!(
            ValueObjectProcessor::derive_name(TargetType::Event),
            "EventValueObject"
        );
        assert_eq!(Ddd4jCodeGenUtils::snake_case("MyEntityId"), "my_entity_id");

        let header = ValueObjectTemplate {
            type_name: "VendorKey".into(),
            description: "Vendor key value object".into(),
        }
        .render_header();
        assert!(header.contains("pub struct VendorKey"));

        let _event_template = EventVoTemplate {
            type_name: "Created".into(),
            fields: vec![Field {
                name: "id".into(),
                rust_type: "EventId".into(),
            }],
        };
        let _clasz = Clasz {
            package: "org.example".into(),
            name: "Created".into(),
        };
    }

    /// 覆盖 `compile_error` 与值对象字段校验失败路径。
    #[test]
    fn compile_error_and_value_object_helpers_cover_failures() {
        let tokens = compile_error(syn::Error::new(
            proc_macro2::Span::call_site(),
            "sample compile error",
        ));
        assert!(!tokens.is_empty());

        let enum_input: DeriveInput = parse_quote! { enum NotAStruct { A } };
        assert!(single_field(&enum_input).is_err());

        let empty: DeriveInput = parse_quote! { struct Empty; };
        assert!(single_field(&empty).is_err());
        assert!(field_access(struct_fields_for_test(&empty)).is_err());

        let two_fields: DeriveInput = parse_quote! { struct Two(i32, i32); };
        assert!(single_field(&two_fields).is_err());

        let named: DeriveInput = parse_quote! { struct Named { value: String } };
        let field = single_field(&named).expect("named");
        assert_eq!(field.ident.as_ref().unwrap().to_string(), "value");
        assert_eq!(
            field_access(struct_fields_for_test(&named))
                .expect("access")
                .to_string(),
            "self . value"
        );

        let tuple: DeriveInput = parse_quote! { struct Tuple(String); };
        assert_eq!(
            field_access(struct_fields_for_test(&tuple))
                .unwrap()
                .to_string(),
            "self . 0"
        );
        let enum_for_fields: DeriveInput = parse_quote! { enum E { A } };
        assert!(std::panic::catch_unwind(|| {
            let _ = struct_fields_for_test(&enum_for_fields);
        })
        .is_err());

        // Manually construct named fields without ident / empty named list.
        let empty_named = Fields::Named(syn::FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new(),
        });
        assert!(field_access(&empty_named).is_err());
        let mut anonymous = syn::punctuated::Punctuated::new();
        anonymous.push(syn::Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Inherited,
            modifiers: syn::FieldModifiers::default(),
            ident: None,
            colon_token: None,
            ty: parse_quote!(i32),
            default: None,
        });
        let anon_named = Fields::Named(syn::FieldsNamed {
            brace_token: Default::default(),
            named: anonymous,
        });
        assert!(field_access(&anon_named).is_err());

        let _ = expand_string_value_object(enum_input);
        let _ = expand_integer_entity_id_value_object(parse_quote! { enum Bad { A } });
        let _ = expand_aggregate_root_uuid_value_object(parse_quote! { struct Empty; });
        let _ = expand_string_value_object(parse_quote! { struct OkName(String); });
        let _ = expand_string_value_object(parse_quote! { struct OkNamed { value: String } });
        assert!(string_value_object_access(&parse_quote! { enum X { A } }).is_err());

        let _ = expand_apply_event(false, parse_quote! { impl Agg {} });
        let _ = expand_apply_event(true, parse_quote! { impl Agg {} });
        let _ = expand_apply_event(
            true,
            parse_quote! {
                impl Agg {
                    fn on_created(&mut self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
                }
            },
        );
        let mut dual_receiver: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&mut self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
            }
        };
        force_second_input_self(&mut dual_receiver);
        force_second_input_self(&mut parse_quote! { impl Agg {} });
        assert!(validate_apply_event_impl(&dual_receiver).is_err());
        let _ = expand_apply_event(true, dual_receiver);

        let _ = expand_child_locator(false, parse_quote! { impl Agg {} });
        let _ = expand_child_locator(true, parse_quote! { impl Agg {} });
        let _ = expand_child_locator(
            true,
            parse_quote! {
                impl Agg {
                    fn locate(&self, id: &Id) -> Option<&Child> { None }
                }
            },
        );
    }

    /// 覆盖 `#[apply_event]` 校验器的全部错误与成功分支。
    #[test]
    fn apply_event_validator_covers_error_and_success_paths() {
        let empty: ItemImpl = parse_quote! { impl Agg {} };
        assert!(validate_apply_event_impl(&empty)
            .unwrap_err()
            .to_string()
            .contains("at least one event handler"));

        let bad_receiver: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
            }
        };
        assert!(validate_apply_event_impl(&bad_receiver).is_err());

        let bad_result: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&mut self, event: &Created) -> () {}
            }
        };
        assert!(validate_apply_event_impl(&bad_result)
            .unwrap_err()
            .to_string()
            .contains("must return Result"));

        let bad_arg: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&mut self, event: Created) -> Result<(), AggregateError> { Ok(()) }
            }
        };
        assert!(validate_apply_event_impl(&bad_arg)
            .unwrap_err()
            .to_string()
            .contains("shared event reference"));

        let bad_path: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&mut self, event: &dyn Created) -> Result<(), AggregateError> { Ok(()) }
            }
        };
        assert!(validate_apply_event_impl(&bad_path)
            .unwrap_err()
            .to_string()
            .contains("concrete event type"));

        let duplicate: ItemImpl = parse_quote! {
            impl Agg {
                fn on_a(&mut self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
                fn on_b(&mut self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
            }
        };
        assert!(validate_apply_event_impl(&duplicate)
            .unwrap_err()
            .to_string()
            .contains("duplicate"));

        let ok: ItemImpl = parse_quote! {
            impl Agg {
                fn on_created(&mut self, event: &Created) -> Result<(), AggregateError> { Ok(()) }
                const N: u8 = 1;
            }
        };
        assert_eq!(validate_apply_event_impl(&ok).unwrap().len(), 1);
    }

    /// 覆盖 `#[child_locator]` 校验器的错误与成功分支。
    #[test]
    fn child_locator_validator_covers_error_and_success_paths() {
        let empty: ItemImpl = parse_quote! { impl Agg {} };
        assert!(validate_child_locator_impl(&empty)
            .unwrap_err()
            .to_string()
            .contains("at least one locator"));

        let bad: ItemImpl = parse_quote! {
            impl Agg {
                fn locate(&mut self, id: &Id) -> Option<&Child> { None }
            }
        };
        assert!(validate_child_locator_impl(&bad).is_err());

        let ok: ItemImpl = parse_quote! {
            impl Agg {
                fn locate(&self, id: &Id) -> Option<&Child> { None }
                const IGNORED: u8 = 1;
            }
        };
        assert!(validate_child_locator_impl(&ok).is_ok());

        assert!(require_empty_attr(true, "unused").is_ok());
        assert!(require_empty_attr(false, "#[apply_event] does not accept arguments").is_err());
        assert!(require_empty_attr(false, "#[child_locator] does not accept arguments").is_err());

        // Keep quote import used for sanity in this module.
        let _ = quote!(child_locator_ok);
    }
}
