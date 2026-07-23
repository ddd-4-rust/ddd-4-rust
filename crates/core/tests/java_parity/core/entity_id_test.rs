//! `entity_id_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::{TestEntityId, TestEntityIdFactory};
use ddd_4_rust_core::EntityId;

#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_typed_identifiers() {
    let factory = TestEntityIdFactory;
    assert!(<TestEntityId as EntityId>::is_valid(&factory, None));
    assert!(<TestEntityId as EntityId>::is_valid(&factory, Some("A 1")));
    assert!(!<TestEntityId as EntityId>::is_valid(&factory, Some("X 1")));
    assert!(!<TestEntityId as EntityId>::is_valid(
        &factory,
        Some("A invalid")
    ));
}

#[test]
/// 执行 `rejects_malformed_typed_identifiers` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_malformed_typed_identifiers() {
    let factory = TestEntityIdFactory;
    assert!(!<TestEntityId as EntityId>::is_valid(&factory, Some("")));
    assert!(!<TestEntityId as EntityId>::is_valid(&factory, Some("A")));
}

#[test]
/// 执行 `converts_typed_identifiers` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn converts_typed_identifiers() {
    let factory = TestEntityIdFactory;
    let id = <TestEntityId as EntityId>::value_of(&factory, Some("B 12"));
    assert_eq!(
        id.map(|value| value.as_typed_string()),
        Some("B 12".to_owned())
    );
    assert!(<TestEntityId as EntityId>::value_of(&factory, Some("X 1")).is_none());
}
