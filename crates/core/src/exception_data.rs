//! `exception_data` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Base for exception data used in marshalling/unmarshalling.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.ExceptionData`.

/// Base for all classes that store data from an exception for marshalling
/// and allowing to recreate it after unmarshalling.
///
/// Java: `ExceptionData<EX extends Exception> extends Serializable, ValueObject, ToExceptionCapable<EX>`
///
/// # Type Parameters
/// - `E`: The concrete Rust error type this data can recreate.
///
/// `ExceptionData` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait ExceptionData<E>: Send + Sync {
    /// Returns the name of the data attribute/element.
    ///
    /// Java: `getDataElement() -> String`
    ///
    /// 执行 `data_element` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn data_element(&self) -> &str;

    /// Recreates the exception/error from this data.
    ///
    /// Java: `toException() -> EX` (from ToExceptionCapable)
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn to_error(&self) -> E;
}
