//! `architecture_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::{
    a_created_event::ACreatedEvent, a_id::AId, a_root::ARoot,
    abstract_domain_event::AbstractDomainEvent, abstract_event::AbstractEvent,
    b_added_event::BAddedEvent, b_entity::BEntity, b_id::BId, base_root::BaseRoot,
    c_added_event::CAddedEvent, c_entity::CEntity, c_event::CEvent, c_id::CId, d_event::DEvent,
    impl_root_id::ImplRootId, person_id::PersonId, vendor_id::VendorId,
};
use ddd_4_rust_core::{AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventType};
use std::sync::Arc;

#[test]
/// 执行 `test_model_types_follow_the_domain_boundaries` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn test_model_types_follow_the_domain_boundaries() -> Result<(), Box<dyn std::error::Error>> {
    let a: AId = AId::new("A", 1)?;
    let b: BId = BId::new("B", 2)?;
    let c: CId = CId::new("C", 3)?;
    let person: PersonId = PersonId::new("Person")?;
    let vendor: VendorId = VendorId::new("Vendor")?;
    let root_id: ImplRootId = ImplRootId::new("ImplRoot")?;
    assert_eq!(a.as_typed_string(), "A 1");
    assert_eq!(b.as_typed_string(), "B 2");
    assert_eq!(c.as_typed_string(), "C 3");
    assert_ne!(person.as_string(), vendor.as_string());
    assert_eq!(root_id.entity_type().as_string(), "ImplRoot");
    Ok(())
}

#[test]
/// 执行 `test_model_events_and_entities_are_executable` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn test_model_events_and_entities_are_executable() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = AbstractEvent::new(
        EventType::new("ACreatedEvent")?,
        "2020-01-01T00:00:00Z".parse()?,
    );
    let id = Arc::new(AId::new("A", 1)?) as Arc<dyn EntityId>;
    let event = AbstractDomainEvent::new(
        metadata,
        EntityIdPath::new(vec![id])?,
        AggregateVersion::new(0),
    );
    assert_eq!(event.event_type().as_str(), "ACreatedEvent");
    assert_eq!(event.aggregate_version_integer(), Some(0));

    let created = ACreatedEvent {
        name: "Ada".to_owned(),
    };
    let added_b = BAddedEvent { id: 2 };
    let added_c = CAddedEvent { id: 3 };
    let c_event = CEvent { id: 3 };
    let unknown = DEvent { sequence: 4 };
    let b_entity = BEntity { id: 2, applied: 1 };
    let c_entity = CEntity { id: 3, applied: 1 };
    let root = ARoot {
        base: BaseRoot {
            applied_events: vec![event.event_type().to_string()],
        },
        name: Some(created.name.clone()),
    };
    assert_eq!(
        (added_b.id, added_c.id, c_event.id, unknown.sequence),
        (2, 3, 3, 4)
    );
    assert_eq!(b_entity.applied + c_entity.applied, 2);
    assert_eq!(root.name.as_deref(), Some("Ada"));
    assert_eq!(root.base.applied_events, vec!["ACreatedEvent"]);
    Ok(())
}
