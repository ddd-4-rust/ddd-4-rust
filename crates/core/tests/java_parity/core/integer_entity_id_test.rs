//! `integer_entity_id_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{EntityId, IntegerEntityId};

#[test]
/// 执行 `implements_the_value_object_contract` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn implements_the_value_object_contract() -> Result<(), Box<dyn std::error::Error>> {
    let first = IntegerEntityId::new("Person", 7)?;
    let second = IntegerEntityId::new("Person", 7)?;
    assert_eq!(first, second);
    assert_eq!(first.as_i32(), 7);
    Ok(())
}

#[test]
/// 执行 `orders_by_type_and_then_identifier` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn orders_by_type_and_then_identifier() -> Result<(), Box<dyn std::error::Error>> {
    let a1 = IntegerEntityId::new("A", 1)?;
    let a2 = IntegerEntityId::new("A", 2)?;
    let b1 = IntegerEntityId::new("B", 1)?;
    assert!(a1 < a2);
    assert!(a2 < b1);
    Ok(())
}

#[test]
/// 执行 `exposes_string_and_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn exposes_string_and_type() -> Result<(), Box<dyn std::error::Error>> {
    let id = IntegerEntityId::new("Person", 42)?;
    assert_eq!(id.as_string(), "42");
    assert_eq!(id.as_typed_string(), "Person 42");
    assert_eq!(id.to_string(), "42");
    Ok(())
}
