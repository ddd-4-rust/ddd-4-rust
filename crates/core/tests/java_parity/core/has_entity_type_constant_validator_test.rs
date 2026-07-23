//! `has_entity_type_constant_validator_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{HasEntityTypeConstant, HasEntityTypeConstantValidator};

/// `Valid` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct Valid;
impl HasEntityTypeConstant for Valid {
    /// `ENTITY_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const ENTITY_TYPE: &'static str = "A";
}
/// `Empty` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct Empty;
impl HasEntityTypeConstant for Empty {
    /// `ENTITY_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const ENTITY_TYPE: &'static str = "";
}
/// `Maximum` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct Maximum;
impl HasEntityTypeConstant for Maximum {
    /// `ENTITY_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const ENTITY_TYPE: &'static str = concat!(
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaa"
    );
}
/// `TooLong` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct TooLong;
impl HasEntityTypeConstant for TooLong {
    /// `ENTITY_TYPE` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    const ENTITY_TYPE: &'static str = concat!(
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa"
    );
}

#[test]
/// 执行 `extracts_valid_constant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn extracts_valid_constant() {
    assert_eq!(
        HasEntityTypeConstantValidator::extract_value::<Valid>().map(|v| v.to_string()),
        Ok("A".to_owned())
    );
}
#[test]
/// 执行 `rejects_empty_constant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_empty_constant() {
    assert!(HasEntityTypeConstantValidator::extract_value::<Empty>().is_err());
}
#[test]
/// 执行 `accepts_maximum_length_constant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn accepts_maximum_length_constant() {
    assert!(HasEntityTypeConstantValidator::extract_value::<Maximum>().is_ok());
}
#[test]
/// 执行 `rejects_overlong_constant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn rejects_overlong_constant() {
    assert!(HasEntityTypeConstantValidator::extract_value::<TooLong>().is_err());
}
#[test]
/// 执行 `exposes_stable_associated_constant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn exposes_stable_associated_constant() {
    assert_eq!(Valid::ENTITY_TYPE, "A");
}
#[test]
/// 执行 `associated_constant_is_static` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn associated_constant_is_static() {
    let first = Valid::ENTITY_TYPE.as_ptr();
    let second = Valid::ENTITY_TYPE.as_ptr();
    assert_eq!(first, second);
}
#[test]
/// 执行 `invalid_constant_preserves_validation_error` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn invalid_constant_preserves_validation_error() {
    assert_eq!(
        HasEntityTypeConstantValidator::extract_value::<Empty>()
            .err()
            .map(|e| e.to_string()),
        Some("entity type must not be empty".to_owned())
    );
}
