//! `encrypted_data_service` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Service for encrypting/decrypting data and handling versioned secret keys.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EncryptedDataService`.

use crate::encrypted_data::EncryptedData;

/// Error type for encryption operations.
#[derive(Debug, thiserror::Error)]
/// `EncryptionError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum EncryptionError {
    /// The given key identifier is unknown.
    #[error("encryption key id unknown: {0}")]
    EncryptionKeyIdUnknown(String),

    /// The given version of the key is unknown.
    #[error("encryption key version unknown: {0}")]
    EncryptionKeyVersionUnknown(String),

    /// A key with the given ID already exists.
    #[error("duplicate encryption key id: {0}")]
    DuplicateEncryptionKeyId(String),

    /// Decrypting the data failed.
    #[error("decryption failed")]
    DecryptionFailed,

    /// Other error.
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

/// Service for encrypting/decrypting `EncryptedData` and handling versioned secret keys.
///
/// Java: `EncryptedDataService`
///
/// `EncryptedDataService` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EncryptedDataService: Send + Sync {
    /// Determines if a key for the given identifier exists.
    ///
    /// Java: `keyExists(String keyId) -> boolean`
    ///
    /// 执行 `key_exists` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_exists(&self, key_id: &str) -> bool;

    /// Creates a new key for the given identifier.
    ///
    /// Java: `createKey(String keyId) throws DuplicateEncryptionKeyIdException`
    ///
    /// 执行 `create_key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create_key(&self, key_id: &str) -> Result<(), EncryptionError>;

    /// Rotates the existing key by creating a new one as the next version.
    ///
    /// Java: `rotateKey(String keyId) -> String throws EncryptionKeyIdUnknownException`
    ///
    /// 执行 `rotate_key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn rotate_key(&self, key_id: &str) -> Result<String, EncryptionError>;

    /// Returns the current version of the given identifier.
    ///
    /// Java: `getKeyVersion(String keyId) -> String throws EncryptionKeyIdUnknownException`
    ///
    /// 执行 `key_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_version(&self, key_id: &str) -> Result<String, EncryptionError>;

    /// Encrypts some data using a dedicated key.
    ///
    /// Java: `encrypt(String keyId, String dataType, String contentType, byte[] data) -> EncryptedData`
    ///
    /// 执行 `encrypt` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn encrypt(
        &self,
        key_id: &str,
        data_type: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<Box<dyn EncryptedData>, EncryptionError>;

    /// Decrypts the data using the information provided by the parameter.
    ///
    /// Java: `decrypt(EncryptedData encryptedData) -> byte[]`
    ///
    /// 执行 `decrypt` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn decrypt(&self, encrypted_data: &dyn EncryptedData) -> Result<Vec<u8>, EncryptionError>;
}
