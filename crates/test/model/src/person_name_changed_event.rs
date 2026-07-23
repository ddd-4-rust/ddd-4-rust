//! `person_name_changed_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{ModelError, PersonId, PersonName, VendorRef, event_metadata::EventMetadata};
use chrono::{DateTime, Utc};
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType,
};
use std::sync::Arc;

/// Domain event emitted when a person's name changes.
///
/// `PersonNameChangedEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct PersonNameChangedEvent {
    metadata: EventMetadata,
    path: EntityIdPath,
    vendor_ref: VendorRef,
    person_id: PersonId,
    old_name: PersonName,
    new_name: PersonName,
    version: AggregateVersion,
}
impl PersonNameChangedEvent {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        vendor_ref: VendorRef,
        person_id: PersonId,
        old_name: PersonName,
        new_name: PersonName,
        version: AggregateVersion,
    ) -> Result<Self, ModelError> {
        let root_id = Arc::new(*vendor_ref.id()) as Arc<dyn EntityId>;
        let child_id = Arc::new(person_id) as Arc<dyn EntityId>;
        Ok(Self {
            metadata: EventMetadata::new("PersonNameChangedEvent")?,
            path: EntityIdPath::new(vec![root_id, child_id])?,
            vendor_ref,
            person_id,
            old_name,
            new_name,
            version,
        })
    }
    #[must_use]
    /// 执行 `vendor_ref` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn vendor_ref(&self) -> &VendorRef {
        &self.vendor_ref
    }
    #[must_use]
    /// 执行 `person_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn person_id(&self) -> PersonId {
        self.person_id
    }
    #[must_use]
    /// 执行 `old_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn old_name(&self) -> &PersonName {
        &self.old_name
    }
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new_name(&self) -> &PersonName {
        &self.new_name
    }
}
impl std::fmt::Display for PersonNameChangedEvent {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Changed person #{} name from '{}' to '{}' for {}",
            self.person_id, self.old_name, self.new_name, self.vendor_ref
        )
    }
}
impl Event for PersonNameChangedEvent {
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_id(&self) -> &EventId {
        &self.metadata.event_id
    }
    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_type(&self) -> &EventType {
        &self.metadata.event_type
    }
    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.metadata.timestamp
    }
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId> {
        self.metadata.correlation_id.as_ref()
    }
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId> {
        self.metadata.causation_id.as_ref()
    }
}
impl DomainEvent<PersonId> for PersonNameChangedEvent {
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &PersonId {
        &self.person_id
    }
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}
impl DomainEvent<dyn EntityId> for PersonNameChangedEvent {
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &(dyn EntityId + 'static) {
        &self.person_id
    }
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}
