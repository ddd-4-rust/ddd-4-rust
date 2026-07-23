//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! Configuration contracts consumed by the DDD proc-macro crate.
//!
//! Every public type records the Java 0.7.0 annotation it replaces.

#![expect(
    clippy::struct_excessive_bools,
    reason = "boolean switches preserve the independent attributes of the Java 0.7.0 annotations"
)]

mod aggregate_root_uuid_vo;
mod event_vo;
mod integer_entity_id_vo;
mod package_info;
mod simple_value_object;
mod string_vo;

pub use aggregate_root_uuid_vo::AggregateRootUuidVo;
pub use event_vo::EventVo;
pub use integer_entity_id_vo::IntegerEntityIdVo;
pub use package_info::JAVA_PACKAGE as JAVA_PACKAGE_NAME;
pub use simple_value_object::SimpleValueObject;
pub use string_vo::StringVo;
