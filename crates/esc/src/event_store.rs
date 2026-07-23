//! `event_store` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Event store abstraction.
//!
//! 1:1 translation of `org.fuin.esc.api.CommonEvent`, `StreamId`, `EventStore`, etc.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Unique stream identifier.
///
/// Java: `StreamId`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// `StreamId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct StreamId(String);

impl StreamId {
    /// Creates a new stream ID.
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Returns the stream name.
    #[must_use]
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StreamId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A common event as stored in the event store.
///
/// Java: `CommonEvent`
#[derive(Debug, Clone)]
/// `CommonEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct CommonEvent {
    /// Unique event identifier.
    ///
    /// 保存 `event_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub event_id: Uuid,
    /// Event type name.
    ///
    /// 保存 `event_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub event_type: String,
    /// Event data as raw bytes/JSON.
    ///
    /// 保存 `data` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub data: Vec<u8>,
    /// Optional metadata.
    ///
    /// 保存 `metadata` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub metadata: Option<Vec<u8>>,
    /// Event creation timestamp.
    ///
    /// 保存 `created` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub created: DateTime<Utc>,
    /// Stream position / event number.
    ///
    /// 保存 `event_number` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub event_number: i64,
}

/// A slice of events read from a stream.
///
/// Java: `StreamEventsSlice`
#[derive(Debug, Clone)]
/// `StreamEventsSlice` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct StreamEventsSlice {
    /// The events in this slice.
    ///
    /// 保存 `events` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub events: Vec<CommonEvent>,
    /// The next event number to read from, or None if end of stream.
    ///
    /// 保存 `next_event_number` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub next_event_number: Option<i64>,
    /// Whether this is the end of the stream.
    ///
    /// 保存 `is_end_of_stream` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub is_end_of_stream: bool,
    /// The last event number in the stream.
    ///
    /// 保存 `last_event_number` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub last_event_number: Option<i64>,
}

/// Event store abstraction.
///
/// Java: `EventStore`
#[async_trait]
/// `EventStore` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EventStore: Send + Sync {
    /// Reads events forward from a stream.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_stream_events_forward(
        &self,
        stream_id: &StreamId,
        start: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError>;

    /// Appends events to a stream.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn append_to_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
        events: Vec<CommonEvent>,
    ) -> Result<(), EventStoreError>;

    /// Deletes a stream.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn delete_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
    ) -> Result<(), EventStoreError>;

    /// Reads all events forward (for projections).
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_all_events_forward(
        &self,
        position: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError>;
}

/// Event store errors.
#[derive(Debug, thiserror::Error)]
/// `EventStoreError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum EventStoreError {
    /// A stream with the requested identifier does not exist.
    #[error("Stream not found: {0}")]
    StreamNotFound(String),
    /// Optimistic concurrency failed because the stream version changed.
    #[error("Wrong expected version: expected {expected}, actual {actual}")]
    WrongExpectedVersion {
        /// Expected stream version.
        expected: i64,
        /// Actual stream version.
        actual: i64,
    },
    /// The stream was already deleted.
    #[error("Stream deleted: {0}")]
    StreamDeleted(String),
    /// The event store could not be reached.
    #[error("Connection error: {0}")]
    Connection(String),
    /// Event data or metadata could not be serialized.
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Another event-store-specific error occurred.
    #[error("{0}")]
    Other(String),
}
