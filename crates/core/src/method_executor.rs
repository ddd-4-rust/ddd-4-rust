//! `method_executor` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.MethodExecutor`.

use thiserror::Error;

/// Failure returned by an explicitly registered Rust method invocation.
#[derive(Debug, Error)]
#[error("failed to invoke method {method}: {source}")]
/// `MethodExecutionError` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct MethodExecutionError<E: std::error::Error + 'static> {
    method: &'static str,
    #[source]
    source: E,
}

/// Executes typed functions in place of Java reflection.
///
/// `MethodExecutor` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct MethodExecutor;

impl MethodExecutor {
    /// Invokes a named typed function and preserves its source error.
    ///
    /// 执行 `invoke` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn invoke<T, R, E>(
        method: &'static str,
        target: &mut T,
        function: impl FnOnce(&mut T) -> Result<R, E>,
    ) -> Result<R, MethodExecutionError<E>>
    where
        E: std::error::Error + 'static,
    {
        function(target).map_err(|source| MethodExecutionError { method, source })
    }

    /// Verifies two optional ordered argument-type lists.
    #[must_use]
    /// 执行 `same` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn same<T: PartialEq>(expected: Option<&[T]>, actual: Option<&[T]>) -> bool {
        expected == actual
    }
}
