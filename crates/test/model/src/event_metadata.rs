//! `event_metadata` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use chrono::{DateTime, Utc};
use ddd_4_rust_core::{EventId, EventType, EventTypeError};

/// `EventMetadata` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct EventMetadata {
    /// 保存 `event_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub event_id: EventId,
    /// 保存 `event_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub event_type: EventType,
    /// 保存 `timestamp` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub timestamp: DateTime<Utc>,
    /// 保存 `correlation_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub correlation_id: Option<EventId>,
    /// 保存 `causation_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub causation_id: Option<EventId>,
}

impl EventMetadata {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(event_type: &'static str) -> Result<Self, EventTypeError> {
        Ok(Self {
            event_id: EventId::new(),
            event_type: EventType::new(event_type)?,
            timestamp: Utc::now(),
            correlation_id: None,
            causation_id: None,
        })
    }
}
