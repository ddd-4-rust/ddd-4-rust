//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! Serde-native serialization for DDD building blocks.
//!
//! JSON serialization is implemented by `serde` and `serde_json`; XML support is
//! feature-gated through `quick-xml`. The frozen Java namespaces exist only as
//! migration facades and do not contain a separate serialization implementation.

#![expect(
    clippy::missing_errors_doc,
    clippy::ref_option_ref,
    reason = "generated compatibility callbacks mirror the frozen Java API and are documented at type level"
)]

mod abstract_domain_event;
mod abstract_event;
mod aggregate_version_adapter;
mod compat;
mod ddd_serde_module;
mod entity_id_adapter;
mod entity_id_path_adapter;
mod zoned_date_time_value;

/// 公开 `json` 子模块，用于组织对应领域类型及其兼容入口。
pub mod json;
#[cfg(feature = "xml")]
/// 公开 `xml` 子模块，用于组织对应领域类型及其兼容入口。
pub mod xml;

pub use abstract_domain_event::AbstractDomainEvent;
pub use abstract_event::AbstractEvent;
pub use aggregate_version_adapter::{
    deserialize_aggregate_version, deserialize_aggregate_version_opt, serialize_aggregate_version,
    serialize_aggregate_version_opt,
};
pub use ddd_serde_module::DddSerdeModule;
pub use entity_id_adapter::{
    deserialize_entity_id_string, serialize_entity_id, serialize_entity_id_opt,
};
pub use entity_id_path_adapter::{
    deserialize_entity_id_path_with_factory, serialize_entity_id_path,
};
pub use zoned_date_time_value::{ZonedDateTimeError, ZonedDateTimeValue};
