//! `expected_entity_id_path_validator_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::TestEntityId;
use ddd_4_rust_core::{
    EntityId, EntityIdPath, ExpectedEntityIdPath, ExpectedEntityIdPathValidator,
};
use std::sync::Arc;

/// 执行 `path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn path(types: &[&str]) -> Result<EntityIdPath, Box<dyn std::error::Error>> {
    let ids = types
        .iter()
        .zip(0_i32..)
        .map(|(kind, index)| {
            TestEntityId::new(kind, index).map(|id| Arc::new(id) as Arc<dyn EntityId>)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(EntityIdPath::new(ids)?)
}

#[test]
/// 执行 `accepts_absent_paths` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn accepts_absent_paths() {
    assert!(ExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A"])).is_valid(None));
}
#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_one_entry_paths() -> Result<(), Box<dyn std::error::Error>> {
    let value = path(&["A"])?;
    assert!(
        ExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A"])).is_valid(Some(&value))
    );
    Ok(())
}
#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_two_entry_paths() -> Result<(), Box<dyn std::error::Error>> {
    let value = path(&["A", "B"])?;
    assert!(
        ExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A", "B"]))
            .is_valid(Some(&value))
    );
    Ok(())
}
#[test]
/// 执行 `rejects_wrong_order_or_length` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_wrong_order_or_length() -> Result<(), Box<dyn std::error::Error>> {
    let value = path(&["B", "A"])?;
    let validator = ExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A", "B"]));
    assert!(!validator.is_valid(Some(&value)));
    Ok(())
}
