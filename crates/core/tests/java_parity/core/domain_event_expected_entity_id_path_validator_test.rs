//! `domain_event_expected_entity_id_path_validator_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::TestEntityId;
use chrono::{DateTime, Utc};
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, DomainEventExpectedEntityIdPathValidator, EntityId,
    EntityIdPath, Event, EventId, EventType, ExpectedEntityIdPath,
};
use std::sync::Arc;

/// `TestDomainEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct TestDomainEvent {
    event_id: EventId,
    event_type: EventType,
    timestamp: DateTime<Utc>,
    path: EntityIdPath,
    version: AggregateVersion,
}

impl TestDomainEvent {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn new(types: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let ids = types
            .iter()
            .zip(0_i32..)
            .map(|(kind, index)| {
                TestEntityId::new(kind, index).map(|id| Arc::new(id) as Arc<dyn EntityId>)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            event_id: EventId::new(),
            event_type: EventType::new("TestDomainEvent")?,
            timestamp: "2020-01-01T00:00:00Z".parse()?,
            path: EntityIdPath::new(ids)?,
            version: AggregateVersion::new(1),
        })
    }
}

impl Event for TestDomainEvent {
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
        &self.timestamp
    }
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId> {
        None
    }
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

impl DomainEvent<dyn EntityId> for TestDomainEvent {
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

#[test]
/// 执行 `accepts_absent_events` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn accepts_absent_events() {
    let validator = DomainEventExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A"]));
    assert!(validator.is_valid(None));
}

#[test]
/// 校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn validates_event_entity_paths() -> Result<(), Box<dyn std::error::Error>> {
    let event = TestDomainEvent::new(&["A", "B"])?;
    let matching =
        DomainEventExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["A", "B"]));
    let wrong =
        DomainEventExpectedEntityIdPathValidator::new(ExpectedEntityIdPath::new(["B", "A"]));
    assert!(matching.is_valid(Some(&event)));
    assert!(!wrong.is_valid(Some(&event)));
    Ok(())
}
