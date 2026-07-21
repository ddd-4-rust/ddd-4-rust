//! Container for encrypted data with key metadata.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EncryptedData`.

/// Container for encrypted data.
///
/// In addition to the data itself, the container has information about the key used
/// to encrypt the data and the format of the data.
///
/// Java: `EncryptedData extends ValueObject, Serializable`
pub trait EncryptedData: Send + Sync {
    /// Returns the unique identifier of the private key used.
    ///
    /// Java: `getKeyId() -> String`
    fn key_id(&self) -> &str;

    /// Returns the version of the private key used.
    ///
    /// Java: `getKeyVersion() -> String`
    fn key_version(&self) -> &str;

    /// Returns the unique type of the data like "UserPersonalData".
    ///
    /// Java: `getDataType() -> String`
    fn data_type(&self) -> &str;

    /// Returns the content type like "application/json; encoding=UTF-8; version=1".
    ///
    /// Java: `getContentType() -> String`
    fn content_type(&self) -> &str;

    /// Returns the encrypted data.
    ///
    /// Java: `getEncryptedData() -> byte[]`
    fn encrypted_data(&self) -> &[u8];
}
