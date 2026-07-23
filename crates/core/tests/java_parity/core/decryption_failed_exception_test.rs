//! `decryption_failed_exception_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::DecryptionFailedException;

#[test]
/// 执行 `formats_cause_message` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn formats_cause_message() {
    assert_eq!(
        DecryptionFailedException::from_cause("invalid tag").to_string(),
        "Decryption failed: invalid tag"
    );
}
