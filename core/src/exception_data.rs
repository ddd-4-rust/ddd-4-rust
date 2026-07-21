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
pub trait ExceptionData<E>: Send + Sync {
    /// Returns the name of the data attribute/element.
    ///
    /// Java: `getDataElement() -> String`
    fn data_element(&self) -> &str;

    /// Recreates the exception/error from this data.
    ///
    /// Java: `toException() -> EX` (from ToExceptionCapable)
    fn to_error(&self) -> E;
}
