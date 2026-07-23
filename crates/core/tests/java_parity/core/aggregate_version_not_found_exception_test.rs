//! `aggregate_version_not_found_exception_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::AggregateVersionNotFoundException;

#[test]
/// 执行 `preserves_requested_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn preserves_requested_version() {
    let error = AggregateVersionNotFoundException::new("Vendor", "42", 3);
    assert_eq!(error.data().version(), 3);
    assert_eq!(
        error.to_string(),
        "Requested version 3 for Vendor (42) does not exist"
    );
}
