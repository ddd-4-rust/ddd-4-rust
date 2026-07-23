//! `entity_not_found_exception_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::EntityNotFoundException;

#[test]
/// 执行 `formats_optional_parent_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn formats_optional_parent_path() {
    let nested = EntityNotFoundException::new(Some("Vendor 1".to_owned()), "Person 2");
    let root = EntityNotFoundException::new(None, "Vendor 1");
    assert_eq!(nested.parent_id_path(), Some("Vendor 1"));
    assert_eq!(nested.to_string(), "Person 2 not found in Vendor 1");
    assert_eq!(root.to_string(), "Vendor 1 not found");
}
