//! `abstract_domain_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Base domain-event implementation with Serde serialization.
//!
//! 1:1 translation of `org.fuin.ddd4j.jackson.AbstractDomainEvent`.

use crate::ZonedDateTimeValue;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType,
};
use serde::Serialize;

/// Base struct for domain events with serde serialization support.
///
/// Users should embed this in their domain event structs.
///
/// Java: `AbstractDomainEvent<ID extends EntityId> extends AbstractEvent implements DomainEvent<ID>`
///
/// # Example
///
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// struct PersonCreatedEvent {
///     #[serde(flatten)]
///     base: AbstractDomainEvent,
///     name: String,
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
/// `AbstractDomainEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractDomainEvent {
    event_id: EventId,
    event_type: EventType,
    event_timestamp: ZonedDateTimeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    causation_id: Option<EventId>,
    entity_id_path: EntityIdPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_version: Option<AggregateVersion>,
}

impl AbstractDomainEvent {
    /// Creates a new abstract domain event.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_timestamp: ZonedDateTimeValue::from_utc(event_timestamp),
            correlation_id,
            causation_id,
            entity_id_path,
            aggregate_version,
        }
    }

    /// Creates a new abstract domain event with defaults.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new_now(
        event_type: EventType,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
    ) -> Self {
        Self {
            event_id: EventId::new(),
            event_type,
            event_timestamp: ZonedDateTimeValue::from_utc(Utc::now()),
            correlation_id: None,
            causation_id: None,
            entity_id_path,
            aggregate_version,
        }
    }

    /// Returns the event ID.
    #[must_use]
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    /// Returns the event type.
    #[must_use]
    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    /// Returns the event timestamp.
    #[must_use]
    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn event_timestamp(&self) -> &DateTime<Utc> {
        self.event_timestamp.instant()
    }

    /// Creates a domain event while preserving the original IANA time zone.
    #[must_use]
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors the frozen Java event constructor"
    )]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new_zoned(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        timezone: Tz,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_timestamp: ZonedDateTimeValue::new(event_timestamp, timezone),
            correlation_id,
            causation_id,
            entity_id_path,
            aggregate_version,
        }
    }

    /// Returns the event timestamp together with its preserved IANA zone.
    #[must_use]
    /// 执行 `zoned_event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn zoned_event_timestamp(&self) -> &ZonedDateTimeValue {
        &self.event_timestamp
    }

    /// Returns the correlation ID.
    #[must_use]
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn correlation_id(&self) -> Option<&EventId> {
        self.correlation_id.as_ref()
    }

    /// Returns the causation ID.
    #[must_use]
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn causation_id(&self) -> Option<&EventId> {
        self.causation_id.as_ref()
    }

    /// Returns the entity ID path.
    #[must_use]
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_id_path(&self) -> &EntityIdPath {
        &self.entity_id_path
    }

    /// Returns the aggregate version.
    #[must_use]
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.aggregate_version.as_ref()
    }
}

impl Event for AbstractDomainEvent {
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_id(&self) -> &EventId {
        &self.event_id
    }

    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_timestamp(&self) -> &DateTime<Utc> {
        self.event_timestamp.instant()
    }

    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId> {
        self.correlation_id.as_ref()
    }

    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId> {
        self.causation_id.as_ref()
    }
}

// Note: DomainEvent<EntityId> requires entity_id() to return &ID (the last ID in the path).
// Since AbstractDomainEvent doesn't know the concrete EntityId type statically,
// this implementation returns the last ID as &dyn EntityId.
impl DomainEvent<dyn EntityId> for AbstractDomainEvent {
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.entity_id_path
    }

    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &dyn EntityId {
        // Return the last entity ID in the path
        self.entity_id_path.last().as_ref()
    }

    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.aggregate_version.as_ref()
    }
}
