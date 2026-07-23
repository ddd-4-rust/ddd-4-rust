//! `string_based_entity_type_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{EntityTypeError, StringBasedEntityType};

#[test]
/// 执行 `accepts_java_boundary_values` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn accepts_java_boundary_values() {
    let value = "a".repeat(StringBasedEntityType::MAX_LENGTH);
    assert_eq!(
        StringBasedEntityType::new(value.clone()).map(|entity_type| entity_type.to_string()),
        Ok(value)
    );
}

#[test]
/// 执行 `rejects_empty_and_overlong_values` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_empty_and_overlong_values() {
    assert_eq!(StringBasedEntityType::new(""), Err(EntityTypeError::Empty));
    assert!(matches!(
        StringBasedEntityType::new("a".repeat(StringBasedEntityType::MAX_LENGTH + 1)),
        Err(EntityTypeError::TooLong { .. })
    ));
}
