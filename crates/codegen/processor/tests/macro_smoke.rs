//! 宏成功路径冒烟测试，覆盖 proc-macro 展开后的运行时 API。

use std::sync::{Arc, OnceLock};

use chrono::Utc;
use ddd_4_rust_codegen_processor::{
    apply_event, child_locator, AggregateRootUuidValueObject, EventValueObject,
    IntegerEntityIdValueObject, StringValueObject,
};
use ddd_4_rust_core::{
    AggregateError, AggregateVersion, ApplyEventHandler, DomainEvent, EntityId, EntityIdPath,
    Event, EventId, EventType, IntegerEntityId,
};

#[derive(StringValueObject)]
/// 字符串值对象宏生成类型。
struct SmokeName(String);

#[derive(IntegerEntityIdValueObject)]
/// 整型实体 ID 值对象宏生成类型。
struct SmokePersonId(i32);

#[derive(AggregateRootUuidValueObject)]
/// UUID 聚合根 ID 值对象宏生成类型。
struct SmokeRootId(uuid::Uuid);

#[derive(EventValueObject)]
/// 事件值对象宏生成类型。
struct SmokeCreated;

struct SmokeChild {
    id: SmokePersonId,
}

struct SmokeAggregate {
    children: Vec<SmokeChild>,
}

#[child_locator]
impl SmokeAggregate {
    /// 按 ID 定位子实体。
    fn locate_child(&self, id: &SmokePersonId) -> Option<&SmokeChild> {
        self.children
            .iter()
            .find(|child| child.id.as_i32() == id.as_i32())
    }
}

#[apply_event]
impl SmokeAggregate {
    /// 应用创建事件。
    fn on_created(&mut self, _event: &SmokeCreated) -> Result<(), AggregateError> {
        Ok(())
    }
}

/// 返回冒烟测试共享的事件 ID。
fn smoke_event_id() -> &'static EventId {
    static ID: OnceLock<EventId> = OnceLock::new();
    ID.get_or_init(EventId::new)
}

/// 返回冒烟测试共享的事件类型。
fn smoke_event_type() -> &'static EventType {
    static TYPE: OnceLock<EventType> = OnceLock::new();
    TYPE.get_or_init(|| EventType::new(SmokeCreated::EVENT_TYPE).expect("event type"))
}

/// 返回冒烟测试共享的实体 ID 路径。
fn smoke_entity_path() -> &'static EntityIdPath {
    static PATH: OnceLock<EntityIdPath> = OnceLock::new();
    PATH.get_or_init(|| {
        let entity = IntegerEntityId::new("Person", 1).expect("entity id");
        EntityIdPath::new(vec![Arc::new(entity) as Arc<dyn EntityId>]).expect("path")
    })
}

/// 返回冒烟测试共享的实体 ID。
fn smoke_entity_id() -> &'static IntegerEntityId {
    static ID: OnceLock<IntegerEntityId> = OnceLock::new();
    ID.get_or_init(|| IntegerEntityId::new("Person", 1).expect("entity id"))
}

impl Event for SmokeCreated {
    /// 返回事件 ID。
    fn event_id(&self) -> &EventId {
        smoke_event_id()
    }

    /// 返回事件类型。
    fn event_type(&self) -> &EventType {
        smoke_event_type()
    }

    /// 返回事件时间戳。
    fn event_timestamp(&self) -> &chrono::DateTime<Utc> {
        static TIMESTAMP: OnceLock<chrono::DateTime<Utc>> = OnceLock::new();
        TIMESTAMP.get_or_init(Utc::now)
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

impl DomainEvent<dyn EntityId> for SmokeCreated {
    /// 返回实体 ID 路径。
    fn entity_id_path(&self) -> &EntityIdPath {
        smoke_entity_path()
    }

    /// 返回实体 ID。
    fn entity_id(&self) -> &(dyn EntityId + 'static) {
        smoke_entity_id()
    }

    /// 返回聚合版本。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        None
    }
}

#[test]
/// 验证各 derive/attribute 宏展开后可正常调用。
fn macro_smoke_exercises_success_paths() {
    let name = SmokeName::new("Ada");
    assert_eq!(name.as_str(), "Ada");
    assert_eq!(name.to_string(), "Ada");

    let person_id = SmokePersonId::new(7);
    assert_eq!(person_id.as_i32(), 7);

    let root_id = SmokeRootId::new();
    assert!(!root_id.as_uuid().is_nil());

    assert_eq!(SmokeCreated::EVENT_TYPE, "SmokeCreated");

    let mut aggregate = SmokeAggregate {
        children: vec![SmokeChild {
            id: SmokePersonId::new(1),
        }],
    };
    assert!(aggregate.locate_child(&SmokePersonId::new(1)).is_some());

    let event = SmokeCreated;
    let erased: &dyn Event = &event;
    assert_eq!(erased.event_type().as_str(), "SmokeCreated");

    assert!(aggregate
        .try_apply_event(&event as &dyn DomainEvent<dyn EntityId>)
        .expect("dispatch"));
}
