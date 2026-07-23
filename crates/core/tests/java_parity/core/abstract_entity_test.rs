//! `abstract_entity_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::TestEntityId;
use ddd_4_rust_core::AbstractEntity;

#[test]
/// 执行 `stores_and_returns_entity_identifier` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn stores_and_returns_entity_identifier() -> Result<(), Box<dyn std::error::Error>> {
    let mut entity = AbstractEntity::<TestEntityId, TestEntityId>::new();
    assert!(entity.id().is_none());
    entity.set_id(Box::new(TestEntityId::new("B", 2)?));
    assert_eq!(entity.id().map(ToString::to_string), Some("2".to_owned()));
    Ok(())
}

#[test]
/// 执行 `stores_and_returns_parent_identifier` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn stores_and_returns_parent_identifier() -> Result<(), Box<dyn std::error::Error>> {
    let mut entity = AbstractEntity::<TestEntityId, TestEntityId>::new();
    assert!(entity.parent_id().is_none());
    entity.set_parent_id(Box::new(TestEntityId::new("A", 1)?));
    assert_eq!(
        entity.parent_id().map(ToString::to_string),
        Some("1".to_owned())
    );
    Ok(())
}
