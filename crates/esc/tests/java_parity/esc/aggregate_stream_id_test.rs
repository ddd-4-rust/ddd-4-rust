//! `aggregate_stream_id_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{AggregateRootUuid, StringBasedEntityType};
use ddd_4_rust_esc::AggregateStreamId;
use uuid::Uuid;

#[test]
/// 执行 `exposes_stream_name_parameter_and_value` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn exposes_stream_name_parameter_and_value() -> Result<(), Box<dyn std::error::Error>> {
    let id = AggregateRootUuid::from_uuid("Vendor", Uuid::nil())?;
    let kind = StringBasedEntityType::new("Vendor")?;
    let stream = AggregateStreamId::new(&kind, "vendorId", &id);
    assert_eq!(stream.name(), "Vendor");
    assert_eq!(stream.param_name(), "vendorId");
    assert_eq!(stream.param_value(), Uuid::nil().to_string());
    assert!(!stream.is_projection());
    Ok(())
}

#[test]
/// 执行 `equality_hash_and_display_use_wire_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn equality_hash_and_display_use_wire_name() -> Result<(), Box<dyn std::error::Error>> {
    let id = AggregateRootUuid::from_uuid("Vendor", Uuid::nil())?;
    let kind = StringBasedEntityType::new("Vendor")?;
    let first = AggregateStreamId::new(&kind, "vendorId", &id);
    let second = AggregateStreamId::new(&kind, "otherName", &id);
    assert_eq!(first, second);
    assert_eq!(first.as_string(), format!("Vendor-{}", Uuid::nil()));
    assert_eq!(first.to_string(), first.as_string());
    Ok(())
}
