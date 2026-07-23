//! `aggregate_version_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::AggregateVersion;

#[test]
/// 执行 `creates_valid_versions` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn creates_valid_versions() {
    for value in [0, 1, i32::MAX as u32] {
        let version = AggregateVersion::new(value);
        assert_eq!(version.as_u32(), value);
        assert_eq!(version.as_i32(), i32::try_from(value).unwrap_or(i32::MAX));
    }
}

#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_integer_versions() {
    assert!(AggregateVersion::is_valid_u32(None));
    assert!(AggregateVersion::is_valid_u32(Some(0)));
}

#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_string_versions() {
    assert!(AggregateVersion::is_valid_str(None));
    assert!(!AggregateVersion::is_valid_str(Some("-1")));
    assert!(!AggregateVersion::is_valid_str(Some("not-a-number")));
}

#[test]
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parses_integer_versions() {
    assert_eq!(
        AggregateVersion::value_of_u32(Some(1)),
        Some(AggregateVersion::new(1))
    );
    assert_eq!(AggregateVersion::value_of_u32(None), None);
}

#[test]
/// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn parses_string_versions() {
    assert_eq!(
        AggregateVersion::value_of_str(Some("1")),
        Some(AggregateVersion::new(1))
    );
    assert_eq!(AggregateVersion::value_of_str(None), None);
}

#[test]
/// 执行 `preserves_ordering_and_display` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn preserves_ordering_and_display() {
    assert!(AggregateVersion::new(2) > AggregateVersion::new(1));
    assert_eq!(AggregateVersion::new(42).to_string(), "42");
}
