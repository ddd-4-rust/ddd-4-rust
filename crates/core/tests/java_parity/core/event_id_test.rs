//! `event_id_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::EventId;
use uuid::Uuid;

#[test]
/// 执行 `constructs_and_formats_event_ids` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn constructs_and_formats_event_ids() -> Result<(), uuid::Error> {
    let uuid = Uuid::parse_str("bb05f34d-4eac-4f6a-b3c2-5c89269720f3")?;
    let event_id = EventId::from_uuid(uuid);
    assert_eq!(event_id.as_uuid(), &uuid);
    assert_eq!(event_id.as_string(), uuid.to_string());
    assert_eq!(event_id.to_string(), uuid.to_string());
    Ok(())
}

#[test]
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parses_valid_values_and_rejects_invalid_values() {
    let value = "bb05f34d-4eac-4f6a-b3c2-5c89269720f3";
    assert_eq!(
        EventId::value_of(value).map(|id| id.as_string()),
        Some(value.to_owned())
    );
    assert_eq!(EventId::value_of("invalid"), None);
}
