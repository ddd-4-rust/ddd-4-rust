//! 覆盖 `Ddd4JConditions` 的成功与失败分支。

use chrono::Utc;
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, EntityId, EntityIdPath, EntityType, Event, EventId, EventType,
    HasEntityTypeConstant, StringBasedEntityType,
};
use ddd_4_rust_test::{ConditionError, Ddd4JConditions};
use std::sync::Arc;

/// 合法领域事件命名，用于成功路径。
struct ValidNamedEvent {
    id: EventId,
    kind: EventType,
    timestamp: chrono::DateTime<Utc>,
    path: EntityIdPath,
    entity: StubEntityId,
    version: AggregateVersion,
}

/// 类型名不以 `Event` 结尾，用于 `domain_event` 失败路径。
struct BadDomainNaming {
    id: EventId,
    kind: EventType,
    timestamp: chrono::DateTime<Utc>,
    path: EntityIdPath,
    entity: StubEntityId,
    version: AggregateVersion,
}

/// 空 `ENTITY_TYPE` 的实体 ID，用于 `entity_id` 失败路径。
#[derive(Debug, Clone)]
struct EmptyEntityTypeId;

struct EmptyEntityTypeMarker;

impl std::fmt::Display for EmptyEntityTypeMarker {
    /// 输出占位实体类型名。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Placeholder")
    }
}

impl EntityType for EmptyEntityTypeMarker {
    /// 返回占位实体类型名。
    fn as_string(&self) -> &str {
        "Placeholder"
    }
}

static EMPTY_ENTITY_TYPE: EmptyEntityTypeMarker = EmptyEntityTypeMarker;

impl std::fmt::Display for EmptyEntityTypeId {
    /// 输出实体 ID 字符串表示。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0")
    }
}

impl EntityId for EmptyEntityTypeId {
    /// 返回实体类型。
    fn entity_type(&self) -> &dyn EntityType {
        &EMPTY_ENTITY_TYPE
    }

    /// 返回字符串形式的 ID。
    fn as_string(&self) -> String {
        "0".into()
    }
}

impl HasEntityTypeConstant for EmptyEntityTypeId {
    const ENTITY_TYPE: &'static str = "";
}

/// 合法非空 `ENTITY_TYPE` 的实体 ID，用于 `entity_id` 成功路径。
#[derive(Debug, Clone)]
struct ValidEntityTypeId;

struct ValidEntityTypeMarker;

impl std::fmt::Display for ValidEntityTypeMarker {
    /// 输出占位实体类型名。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person")
    }
}

impl EntityType for ValidEntityTypeMarker {
    /// 返回占位实体类型名。
    fn as_string(&self) -> &str {
        "Person"
    }
}

static VALID_ENTITY_TYPE: ValidEntityTypeMarker = ValidEntityTypeMarker;

impl std::fmt::Display for ValidEntityTypeId {
    /// 输出实体 ID 字符串表示。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1")
    }
}

impl EntityId for ValidEntityTypeId {
    /// 返回实体类型。
    fn entity_type(&self) -> &dyn EntityType {
        &VALID_ENTITY_TYPE
    }

    /// 返回字符串形式的 ID。
    fn as_string(&self) -> String {
        "1".into()
    }
}

impl HasEntityTypeConstant for ValidEntityTypeId {
    const ENTITY_TYPE: &'static str = "Person";
}

#[derive(Debug, Clone)]
struct StubEntityId {
    value: String,
    entity_type: StringBasedEntityType,
}

impl StubEntityId {
    /// 构造测试用实体 ID。
    fn person(value: &str) -> Self {
        Self {
            value: value.into(),
            entity_type: StringBasedEntityType::new("Person").expect("type"),
        }
    }
}

impl std::fmt::Display for StubEntityId {
    /// 输出实体 ID 字符串表示。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl EntityId for StubEntityId {
    /// 返回实体类型。
    fn entity_type(&self) -> &dyn EntityType {
        &self.entity_type
    }

    /// 返回字符串形式的 ID。
    fn as_string(&self) -> String {
        self.value.clone()
    }
}

impl Event for ValidNamedEvent {
    /// 返回事件 ID。
    fn event_id(&self) -> &EventId {
        &self.id
    }

    /// 返回事件类型。
    fn event_type(&self) -> &EventType {
        &self.kind
    }

    /// 返回事件时间戳。
    fn event_timestamp(&self) -> &chrono::DateTime<Utc> {
        &self.timestamp
    }

    /// 返回关联 ID。
    fn correlation_id(&self) -> Option<&EventId> {
        None
    }

    /// 返回因果 ID。
    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

impl DomainEvent<StubEntityId> for ValidNamedEvent {
    /// 返回实体 ID 路径。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }

    /// 返回实体 ID。
    fn entity_id(&self) -> &StubEntityId {
        &self.entity
    }

    /// 返回聚合版本。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}

impl Event for BadDomainNaming {
    /// 返回事件 ID。
    fn event_id(&self) -> &EventId {
        &self.id
    }

    /// 返回事件类型。
    fn event_type(&self) -> &EventType {
        &self.kind
    }

    /// 返回事件时间戳。
    fn event_timestamp(&self) -> &chrono::DateTime<Utc> {
        &self.timestamp
    }

    /// 返回关联 ID。
    fn correlation_id(&self) -> Option<&EventId> {
        None
    }

    /// 返回因果 ID。
    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

impl DomainEvent<StubEntityId> for BadDomainNaming {
    /// 返回实体 ID 路径。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }

    /// 返回实体 ID。
    fn entity_id(&self) -> &StubEntityId {
        &self.entity
    }

    /// 返回聚合版本。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}

/// 构造通用测试事件骨架。
fn sample_event<T>() -> (EventId, EventType, chrono::DateTime<Utc>, EntityIdPath, StubEntityId, AggregateVersion)
{
    let entity = StubEntityId::person("1");
    let path = EntityIdPath::new(vec![Arc::new(entity.clone()) as Arc<dyn EntityId>])
        .expect("path");
    (
        EventId::new(),
        EventType::new("Sample").expect("type"),
        Utc::now(),
        path,
        entity,
        AggregateVersion::new(0),
    )
}

#[test]
/// 覆盖 `Ddd4JConditions::domain_event` 成功与失败路径。
fn ddd4j_conditions_domain_event_paths() {
    assert!(Ddd4JConditions::domain_event::<ValidNamedEvent, StubEntityId>().is_ok());

    let (id, kind, timestamp, path, entity, version) = sample_event::<()>();
    let bad = BadDomainNaming {
        id,
        kind,
        timestamp,
        path,
        entity,
        version,
    };
    let _ = bad;
    let error = Ddd4JConditions::domain_event::<BadDomainNaming, StubEntityId>()
        .expect_err("bad naming");
    assert!(matches!(error, ConditionError { .. }));
    assert!(error.to_string().contains("domain event naming"));
}

#[test]
/// 覆盖 `Ddd4JConditions::entity_id` 成功与失败路径。
fn ddd4j_conditions_entity_id_paths() {
    assert!(Ddd4JConditions::entity_id::<ValidEntityTypeId>().is_ok());
    assert!(Ddd4JConditions::entity_id::<EmptyEntityTypeId>()
        .expect_err("empty type")
        .to_string()
        .contains("ENTITY_TYPE"));
}
