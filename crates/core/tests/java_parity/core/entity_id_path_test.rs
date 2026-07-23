//! `entity_id_path_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::{TestEntityId, TestEntityIdFactory};
use ddd_4_rust_core::{EntityId, EntityIdPath, EntityIdPathError};
use std::sync::Arc;

/// 执行 `path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn path(values: &[(&str, i32)]) -> Result<EntityIdPath, Box<dyn std::error::Error>> {
    let ids = values
        .iter()
        .map(|(kind, value)| {
            TestEntityId::new(kind, *value).map(|id| Arc::new(id) as Arc<dyn EntityId>)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(EntityIdPath::new(ids)?)
}

#[test]
/// 执行 `rejects_empty_paths` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_empty_paths() {
    assert_eq!(
        EntityIdPath::new(Vec::new()).err(),
        Some(EntityIdPathError::Empty)
    );
}
#[test]
/// 执行 `creates_single_entry_paths` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn creates_single_entry_paths() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1)])?.len(), 1);
    Ok(())
}
#[test]
/// 执行 `creates_multiple_entry_paths` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn creates_multiple_entry_paths() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1), ("B", 2)])?.len(), 2);
    Ok(())
}
#[test]
/// 执行 `returns_first_entry` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn returns_first_entry() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        path(&[("A", 1), ("B", 2)])?.first().as_typed_string(),
        "A 1"
    );
    Ok(())
}
#[test]
/// 执行 `returns_last_entry` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn returns_last_entry() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1), ("B", 2)])?.last().as_typed_string(), "B 2");
    Ok(())
}
#[test]
/// 执行 `single_entry_has_no_rest` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn single_entry_has_no_rest() -> Result<(), Box<dyn std::error::Error>> {
    assert!(path(&[("A", 1)])?.rest().is_none());
    Ok(())
}
#[test]
/// 执行 `returns_rest` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn returns_rest() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        path(&[("A", 1), ("B", 2)])?.rest().map(|p| p.to_string()),
        Some("B 2".to_owned())
    );
    Ok(())
}
#[test]
/// 执行 `single_entry_has_no_parent` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn single_entry_has_no_parent() -> Result<(), Box<dyn std::error::Error>> {
    assert!(path(&[("A", 1)])?.parent().is_none());
    Ok(())
}
#[test]
/// 执行 `returns_parent` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn returns_parent() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        path(&[("A", 1), ("B", 2)])?.parent().map(|p| p.to_string()),
        Some("A 1".to_owned())
    );
    Ok(())
}
#[test]
/// 执行 `reports_size` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn reports_size() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1), ("B", 2), ("C", 3)])?.size(), 3);
    Ok(())
}
#[test]
/// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn serializes_typed_path() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1), ("B", 2)])?.as_base_type(), "A 1/B 2");
    Ok(())
}
#[test]
/// 执行 `displays_typed_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn displays_typed_path() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(path(&[("A", 1)])?.to_string(), "A 1");
    Ok(())
}
#[test]
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parses_typed_path() {
    assert_eq!(
        EntityIdPath::value_of(&TestEntityIdFactory, Some("A 1/B 2")).map(|p| p.to_string()),
        Some("A 1/B 2".to_owned())
    );
}
#[test]
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parses_absent_path_as_none() {
    assert!(EntityIdPath::value_of(&TestEntityIdFactory, None).is_none());
}
#[test]
/// 执行 `rejects_unknown_path_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_unknown_path_type() {
    assert!(EntityIdPath::value_of(&TestEntityIdFactory, Some("X 1")).is_none());
}
#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_absent_path() {
    assert!(EntityIdPath::is_valid(&TestEntityIdFactory, None));
}
#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_well_formed_path() {
    assert!(EntityIdPath::is_valid(
        &TestEntityIdFactory,
        Some("A 1/B 2")
    ));
}
#[test]
/// 执行 `rejects_malformed_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_malformed_path() {
    assert!(!EntityIdPath::is_valid(
        &TestEntityIdFactory,
        Some("A 1/X 2")
    ));
}
