//! `aggregate_not_found_exception_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::AggregateNotFoundException;

#[test]
/// 执行 `preserves_type_id_and_message` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn preserves_type_id_and_message() {
    let error = AggregateNotFoundException::new("Vendor", "42");
    assert_eq!(error.data().entity_type(), "Vendor");
    assert_eq!(error.data().entity_id(), "42");
    assert_eq!(error.to_string(), "Vendor with id 42 not found");
}
