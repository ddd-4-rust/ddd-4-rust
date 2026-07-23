//! `encryption_key_id_unknown_exception_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::EncryptionKeyIdUnknownException;

#[test]
/// 执行 `preserves_key_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn preserves_key_id() {
    let error = EncryptionKeyIdUnknownException::new("missing");
    assert_eq!(error.key_id(), "missing");
    assert_eq!(error.to_string(), "Unknown keyId: missing");
}
