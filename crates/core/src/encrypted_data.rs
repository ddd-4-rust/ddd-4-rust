//! `encrypted_data` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Container for encrypted data with key metadata.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EncryptedData`.

/// Container for encrypted data.
///
/// In addition to the data itself, the container has information about the key used
/// to encrypt the data and the format of the data.
///
/// Java: `EncryptedData extends ValueObject, Serializable`
///
/// `EncryptedData` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EncryptedData: Send + Sync {
    /// Returns the unique identifier of the private key used.
    ///
    /// Java: `getKeyId() -> String`
    ///
    /// 执行 `key_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_id(&self) -> &str;

    /// Returns the version of the private key used.
    ///
    /// Java: `getKeyVersion() -> String`
    ///
    /// 执行 `key_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_version(&self) -> &str;

    /// Returns the unique type of the data like "UserPersonalData".
    ///
    /// Java: `getDataType() -> String`
    ///
    /// 执行 `data_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn data_type(&self) -> &str;

    /// Returns the content type like "application/json; encoding=UTF-8; version=1".
    ///
    /// Java: `getContentType() -> String`
    ///
    /// 执行 `content_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn content_type(&self) -> &str;

    /// Returns the encrypted data.
    ///
    /// Java: `getEncryptedData() -> byte[]`
    ///
    /// 执行 `encrypted_data` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn encrypted_data(&self) -> &[u8];
}
