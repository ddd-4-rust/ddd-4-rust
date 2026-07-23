//! `abstract_domain_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use chrono::{DateTime, Utc};
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType,
};

/// Shared domain-event data matching the Java fixture inheritance layer.
///
/// `AbstractDomainEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractDomainEvent {
    metadata: super::abstract_event::AbstractEvent,
    path: EntityIdPath,
    version: AggregateVersion,
}

impl AbstractDomainEvent {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        metadata: super::abstract_event::AbstractEvent,
        path: EntityIdPath,
        version: AggregateVersion,
    ) -> Self {
        Self {
            metadata,
            path,
            version,
        }
    }
}

impl Event for AbstractDomainEvent {
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_id(&self) -> &EventId {
        self.metadata.event_id()
    }
    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_type(&self) -> &EventType {
        self.metadata.event_type()
    }
    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_timestamp(&self) -> &DateTime<Utc> {
        self.metadata.event_timestamp()
    }
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId> {
        self.metadata.correlation_id()
    }
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId> {
        self.metadata.causation_id()
    }
}

impl DomainEvent<dyn EntityId> for AbstractDomainEvent {
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &(dyn EntityId + 'static) {
        self.path.last().as_ref()
    }
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}
