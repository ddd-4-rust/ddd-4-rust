//! `jandex_entity_id_factory_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::TestEntityId;
use ddd_4_rust_core::{EntityId, EntityIdFactory, EntityIdRegistration, JandexEntityIdFactory};

/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parse_a(value: &str) -> Option<Box<dyn EntityId>> {
    value.parse::<i32>().ok().and_then(|value| {
        TestEntityId::new("A", value)
            .ok()
            .map(|id| Box::new(id) as Box<dyn EntityId>)
    })
}

/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validate_a(value: &str) -> bool {
    value.parse::<i32>().is_ok()
}

inventory::submit! {
    EntityIdRegistration { entity_type: "A", parse: parse_a, validate: validate_a }
}

#[test]
/// 执行 `lists_registered_identifier_types` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn lists_registered_identifier_types() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(JandexEntityIdFactory::new()?.id_classes(), vec!["A"]);
    Ok(())
}
#[test]
/// 执行 `creates_registered_identifiers` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn creates_registered_identifiers() -> Result<(), Box<dyn std::error::Error>> {
    let factory = JandexEntityIdFactory::new()?;
    assert_eq!(
        factory
            .create_entity_id("A", "7")
            .map(|id| id.as_typed_string()),
        Some("A 7".to_owned())
    );
    Ok(())
}
#[test]
/// 执行 `reports_registered_types` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn reports_registered_types() -> Result<(), Box<dyn std::error::Error>> {
    let factory = JandexEntityIdFactory::new()?;
    assert!(factory.contains_type("A"));
    assert!(!factory.contains_type("X"));
    Ok(())
}
#[test]
/// 执行 `delegates_identifier_validation` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn delegates_identifier_validation() -> Result<(), Box<dyn std::error::Error>> {
    let factory = JandexEntityIdFactory::new()?;
    assert!(factory.is_valid("A", "7"));
    assert!(!factory.is_valid("A", "invalid"));
    assert!(!factory.is_valid("X", "7"));
    Ok(())
}
