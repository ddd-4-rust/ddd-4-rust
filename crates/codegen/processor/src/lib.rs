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
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
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
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod value_object_processor;
#[expect(
    dead_code,
    reason = "frozen Java template configuration is exercised by golden tests"
)]
mod value_object_template;

/// 执行 `compile_error` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn compile_error(error: syn::Error) -> TokenStream {
    error.into_compile_error().into()
}

/// Generates `ApplyEventHandler` dispatch for checked `Result` event handlers.
#[proc_macro_attribute]
/// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn apply_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return compile_error(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[apply_event] does not accept arguments",
        ));
    }
    let input = parse_macro_input!(item as ItemImpl);
    let mut handlers = Vec::new();
    let mut event_types = BTreeSet::new();
    for item in &input.items {
        let ImplItem::Fn(method) = item else { continue };
        if method.sig.inputs.len() != 2
            || !matches!(method.sig.inputs.first(), Some(FnArg::Receiver(receiver)) if matches!(&receiver.kind, syn::ReceiverKind::Reference(_, _, Some(_))))
        {
            return compile_error(syn::Error::new_spanned(
                &method.sig,
                "event handler must have the form fn handler(&mut self, event: &Event) -> Result<(), AggregateError>",
            ));
        }
        if !matches!(method.sig.output, ReturnType::Type(_, ref output) if matches!(&**output, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Result")))
        {
            return compile_error(syn::Error::new_spanned(
                &method.sig.output,
                "event handler must return Result",
            ));
        }
        let Some(FnArg::Typed(argument)) = method.sig.inputs.iter().nth(1) else {
            continue;
        };
        let Type::Reference(reference) = &*argument.ty else {
            return compile_error(syn::Error::new_spanned(
                &argument.ty,
                "event handler argument must be a shared event reference",
            ));
        };
        let Type::Path(event_type) = &*reference.elem else {
            return compile_error(syn::Error::new_spanned(
                &reference.elem,
                "event handler argument must name a concrete event type",
            ));
        };
        let event_key = quote!(#event_type).to_string();
        if !event_types.insert(event_key) {
            return compile_error(syn::Error::new_spanned(
                &argument.ty,
                "duplicate event handler for this event type",
            ));
        }
        handlers.push((&method.sig.ident, &event_type.path));
    }
    if handlers.is_empty() {
        return compile_error(syn::Error::new_spanned(
            &input,
            "#[apply_event] requires at least one event handler",
        ));
    }
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
    .into()
}

/// Validates an impl block containing child-entity locator methods.
#[proc_macro_attribute]
/// 执行 `child_locator` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn child_locator(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return compile_error(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[child_locator] does not accept arguments",
        ));
    }
    let input = parse_macro_input!(item as ItemImpl);
    let methods: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .collect();
    if methods.is_empty() {
        return compile_error(syn::Error::new_spanned(
            &input,
            "#[child_locator] requires at least one locator method",
        ));
    }
    for method in methods {
        let valid_receiver = matches!(method.sig.inputs.first(), Some(FnArg::Receiver(receiver)) if matches!(&receiver.kind, syn::ReceiverKind::Reference(_, _, None)));
        let valid_result = matches!(method.sig.output, ReturnType::Type(_, ref output) if matches!(&**output, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "Option")));
        if method.sig.inputs.len() != 2 || !valid_receiver || !valid_result {
            return compile_error(syn::Error::new_spanned(
                &method.sig,
                "child locator must have the form fn locator(&self, id: &Id) -> Option<&Entity>",
            ));
        }
    }
    quote!(#input).into()
}

/// 执行 `single_field` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
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

/// 执行 `field_access` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn field_access(fields: &Fields) -> Result<proc_macro2::TokenStream, syn::Error> {
    match fields {
        Fields::Named(named) => named
            .named
            .first()
            .and_then(|field| field.ident.as_ref())
            .map(|ident| quote!(self.#ident))
            .ok_or_else(|| {
                syn::Error::new_spanned(fields, "value object requires one named field")
            }),
        Fields::Unnamed(_) => Ok(quote!(self.0)),
        Fields::Unit => Err(syn::Error::new_spanned(
            fields,
            "value object cannot be a unit struct",
        )),
    }
}

/// Generates a checked string-value-object API for a one-field struct.
#[proc_macro_derive(StringValueObject)]
/// 执行 `string_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn string_value_object(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    if let Err(error) = single_field(&input) {
        return compile_error(error);
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let access = match &input.data {
        Data::Struct(data) => match field_access(&data.fields) {
            Ok(access) => access,
            Err(error) => return compile_error(error),
        },
        _ => {
            return compile_error(syn::Error::new_spanned(
                &input,
                "value-object derives require a struct",
            ));
        }
    };
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "创建生成的字符串值对象，并把输入转换为对象持有的稳定字符串。"] pub fn new(value: impl Into<::std::string::String>) -> Self { Self(value.into()) } #[doc = "以借用形式返回生成对象保存的字符串，不复制数据也不修改对象状态。"] pub fn as_str(&self) -> &str { &#access } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { formatter.write_str(&#access) } } }.into()
}

/// Generates an integer entity-ID value-object API for a one-field tuple struct.
#[proc_macro_derive(IntegerEntityIdValueObject)]
/// 执行 `integer_entity_id_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn integer_entity_id_value_object(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    if let Err(error) = single_field(&input) {
        return compile_error(error);
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "根据给定整数创建生成的实体标识，保持值对象的强类型边界。"] pub const fn new(value: i32) -> Self { Self(value) } #[doc = "返回实体标识保存的整数值，不修改当前对象。"] pub const fn as_i32(&self) -> i32 { self.0 } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(formatter, "{}", self.0) } } }.into()
}

/// Generates a UUID aggregate-root-ID value-object API for a one-field tuple struct.
#[proc_macro_derive(AggregateRootUuidValueObject)]
/// 执行 `aggregate_root_uuid_value_object` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn aggregate_root_uuid_value_object(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    if let Err(error) = single_field(&input) {
        return compile_error(error);
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! { impl #impl_generics #name #ty_generics #where_clause { #[doc = "生成随机 UUID 并创建新的聚合根标识，适用于新聚合的身份初始化。"] pub fn new() -> Self { Self(::uuid::Uuid::new_v4()) } #[doc = "根据已有 UUID 创建聚合根标识，用于反序列化和历史数据恢复。"] pub const fn from_uuid(value: ::uuid::Uuid) -> Self { Self(value) } #[doc = "以借用形式返回聚合根标识中的 UUID，不复制数据也不改变状态。"] pub const fn as_uuid(&self) -> &::uuid::Uuid { &self.0 } } impl #impl_generics ::std::default::Default for #name #ty_generics #where_clause { fn default() -> Self { Self::new() } } impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause { fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(formatter, "{}", self.0) } } }.into()
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
