//! `aggregate_root_uuid_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{AggregateRootUuid, EntityId};
use uuid::Uuid;

#[test]
/// 执行 `implements_the_value_object_contract` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn implements_the_value_object_contract() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::parse_str("bb05f34d-4eac-4f6a-b3c2-5c89269720f3")?;
    let first = AggregateRootUuid::from_uuid("Person", uuid)?;
    let second = AggregateRootUuid::from_uuid("Person", uuid)?;
    assert_eq!(first, second);
    assert_eq!(first.as_string(), uuid.to_string());
    assert_eq!(first.as_typed_string(), format!("Person {uuid}"));
    Ok(())
}

#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_uuid_strings() {
    assert!(AggregateRootUuid::is_valid(None));
    assert!(AggregateRootUuid::is_valid(Some(
        "bb05f34d-4eac-4f6a-b3c2-5c89269720f3"
    )));
    assert!(!AggregateRootUuid::is_valid(Some("")));
    assert!(!AggregateRootUuid::is_valid(Some("0")));
}

#[test]
/// 执行 `rejects_invalid_entity_types` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_invalid_entity_types() {
    assert!(AggregateRootUuid::new("").is_err());
}
