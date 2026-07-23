//! `exceptions` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Non-mapped infrastructure wrapper for independently mapped Java exceptions.

use crate::{
    AggregateAlreadyExistsException, AggregateDeletedException, AggregateNotFoundException,
    AggregateVersionConflictException, AggregateVersionNotFoundException,
    DecryptionFailedException, DuplicateEncryptionKeyIdException, DuplicateEntityException,
    EncryptionKeyIdUnknownException, EncryptionKeyVersionUnknownException, EntityNotFoundException,
};
use thiserror::Error;

/// Unified error returned by aggregate and repository operations.
#[derive(Debug, Error)]
/// `AggregateError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum AggregateError {
    /// Aggregate already exists.
    #[error(transparent)]
    AggregateAlreadyExists(#[from] AggregateAlreadyExistsException),
    /// Aggregate has been deleted.
    #[error(transparent)]
    AggregateDeleted(#[from] AggregateDeletedException),
    /// Aggregate was not found.
    #[error(transparent)]
    AggregateNotFound(#[from] AggregateNotFoundException),
    /// Aggregate version conflicts with storage.
    #[error(transparent)]
    AggregateVersionConflict(#[from] AggregateVersionConflictException),
    /// Aggregate version was not found.
    #[error(transparent)]
    AggregateVersionNotFound(#[from] AggregateVersionNotFoundException),
    /// Decryption failed.
    #[error(transparent)]
    DecryptionFailed(#[from] DecryptionFailedException),
    /// Encryption key identifier already exists.
    #[error(transparent)]
    DuplicateEncryptionKeyId(#[from] DuplicateEncryptionKeyIdException),
    /// Entity already exists.
    #[error(transparent)]
    DuplicateEntity(#[from] DuplicateEntityException),
    /// Encryption key identifier is unknown.
    #[error(transparent)]
    EncryptionKeyIdUnknown(#[from] EncryptionKeyIdUnknownException),
    /// Encryption key version is unknown.
    #[error(transparent)]
    EncryptionKeyVersionUnknown(#[from] EncryptionKeyVersionUnknownException),
    /// Entity was not found.
    #[error(transparent)]
    EntityNotFound(#[from] EntityNotFoundException),
    /// No aggregate or child handler accepted a domain event.
    #[error("No event handler found for event type: {event_type}")]
    EventHandlerNotFound {
        /// Event type that could not be handled.
        event_type: String,
    },
    /// Infrastructure failure that has no Java domain equivalent.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
