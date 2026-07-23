//! `method_executor_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::MethodExecutor;
use std::io;

#[test]
/// 执行 `invokes_registered_method` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn invokes_registered_method() -> Result<(), Box<dyn std::error::Error>> {
    let mut value = 1;
    let result = MethodExecutor::invoke("increment", &mut value, |target| {
        *target += 1;
        Ok::<_, io::Error>(*target)
    })?;
    assert_eq!(result, 2);
    Ok(())
}
#[test]
/// 执行 `preserves_invocation_errors` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn preserves_invocation_errors() {
    let mut value = 0;
    let result = MethodExecutor::invoke("fail", &mut value, |_| {
        Err::<(), _>(io::Error::other("failure"))
    });
    assert_eq!(
        result.err().map(|error| error.to_string()),
        Some("failed to invoke method fail: failure".to_owned())
    );
}
#[test]
/// 执行 `compares_optional_argument_lists` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn compares_optional_argument_lists() {
    assert!(MethodExecutor::same(Some(&["A", "B"]), Some(&["A", "B"])));
    assert!(!MethodExecutor::same(Some(&["A"]), Some(&["B"])));
    assert!(MethodExecutor::same::<&str>(None, None));
}
