//! Service for encrypting/decrypting data and handling versioned secret keys.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EncryptedDataService`.

use crate::encrypted_data::EncryptedData;

/// Error type for encryption operations.
#[derive(Debug, thiserror::Error)]
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
pub trait EncryptedDataService: Send + Sync {
    /// Determines if a key for the given identifier exists.
    ///
    /// Java: `keyExists(String keyId) -> boolean`
    fn key_exists(&self, key_id: &str) -> bool;

    /// Creates a new key for the given identifier.
    ///
    /// Java: `createKey(String keyId) throws DuplicateEncryptionKeyIdException`
    fn create_key(&self, key_id: &str) -> Result<(), EncryptionError>;

    /// Rotates the existing key by creating a new one as the next version.
    ///
    /// Java: `rotateKey(String keyId) -> String throws EncryptionKeyIdUnknownException`
    fn rotate_key(&self, key_id: &str) -> Result<String, EncryptionError>;

    /// Returns the current version of the given identifier.
    ///
    /// Java: `getKeyVersion(String keyId) -> String throws EncryptionKeyIdUnknownException`
    fn key_version(&self, key_id: &str) -> Result<String, EncryptionError>;

    /// Encrypts some data using a dedicated key.
    ///
    /// Java: `encrypt(String keyId, String dataType, String contentType, byte[] data) -> EncryptedData`
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
    fn decrypt(&self, encrypted_data: &dyn EncryptedData) -> Result<Vec<u8>, EncryptionError>;
}
