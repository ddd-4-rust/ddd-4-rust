//! 通过 trait 对象与 typed DomainEvent 调用事件契约，补齐 model crate 覆盖。

use ddd_4_rust_core::{AggregateVersion, DomainEvent, EntityId, Event};
use ddd_4_rust_test_model::{
    PersonCreatedEvent, PersonId, PersonName, PersonNameChangedEvent, VendorCreatedEvent,
    VendorId, VendorKey, VendorName, VendorRef,
};
use uuid::Uuid;

/// 构造测试用供应商引用。
fn vendor_ref() -> Result<VendorRef, Box<dyn std::error::Error>> {
    Ok(VendorRef::new(
        VendorId::from_uuid(Uuid::nil()),
        VendorKey::new("V00001")?,
        VendorName::new("Acme")?,
    ))
}

/// 通过 `&dyn Event` 调用事件元数据 getter。
fn assert_event_via_trait_object(event: &dyn Event) {
    let _ = event.event_id();
    let _ = event.event_timestamp();
    let _ = event.event_type();
    let _ = event.correlation_id();
    let _ = event.causation_id();
}

/// 通过 `&dyn DomainEvent<dyn EntityId>` 调用路径与版本 getter。
fn assert_domain_event_via_trait_object(event: &dyn DomainEvent<dyn EntityId>) {
    let _ = event.entity_id_path();
    let _ = event.entity_id();
    let _ = event.aggregate_version();
}

#[test]
/// 覆盖 `PersonCreatedEvent` 的 trait 对象与 typed DomainEvent 路径。
fn person_created_event_trait_object_coverage() -> Result<(), Box<dyn std::error::Error>> {
    let reference = vendor_ref()?;
    let created = PersonCreatedEvent::builder()
        .vendor_ref(reference)
        .person_id(PersonId::new(1))
        .person_name(PersonName::new("Peter")?)
        .aggregate_version(AggregateVersion::new(1))
        .build()?;

    let event: &dyn Event = &created;
    assert_eq!(event.event_type().as_str(), "PersonCreatedEvent");
    assert_event_via_trait_object(event);

    let domain: &dyn DomainEvent<dyn EntityId> = &created;
    assert_eq!(domain.entity_id_path().size(), 1);
    assert_eq!(domain.aggregate_version().map(|v| v.as_u32()), Some(1));
    assert_domain_event_via_trait_object(domain);
    assert_eq!(
        <PersonCreatedEvent as DomainEvent<PersonId>>::entity_id(&created).as_string(),
        "1"
    );
    assert_eq!(
        format!("{}", created.person_id().entity_type()),
        "Person"
    );
    Ok(())
}

#[test]
/// 覆盖 `PersonNameChangedEvent` 的 trait 对象与 typed DomainEvent 路径。
fn person_name_changed_event_trait_object_coverage() -> Result<(), Box<dyn std::error::Error>> {
    let reference = vendor_ref()?;
    let changed = PersonNameChangedEvent::new(
        reference,
        PersonId::new(1),
        PersonName::new("Peter")?,
        PersonName::new("Harry")?,
        AggregateVersion::new(2),
    )?;

    assert_event_via_trait_object(&changed);
    let domain: &dyn DomainEvent<dyn EntityId> = &changed;
    assert_eq!(domain.entity_id_path().size(), 2);
    assert_eq!(domain.aggregate_version().map(|v| v.as_u32()), Some(2));
    assert_domain_event_via_trait_object(domain);
    assert_eq!(
        <PersonNameChangedEvent as DomainEvent<PersonId>>::entity_id(&changed).as_string(),
        "1"
    );
    Ok(())
}

#[test]
/// 覆盖 `VendorCreatedEvent` 的 trait 对象与 typed DomainEvent 路径。
fn vendor_created_event_trait_object_coverage() -> Result<(), Box<dyn std::error::Error>> {
    let reference = vendor_ref()?;
    let created = VendorCreatedEvent::new(reference)?;

    assert_event_via_trait_object(&created);
    let domain: &dyn DomainEvent<dyn EntityId> = &created;
    assert_eq!(domain.entity_id_path().size(), 1);
    assert_eq!(domain.aggregate_version().map(|v| v.as_u32()), Some(0));
    assert_domain_event_via_trait_object(domain);
    assert_eq!(
        <VendorCreatedEvent as DomainEvent<VendorId>>::entity_id(&created).as_string(),
        Uuid::nil().to_string()
    );
    assert_eq!(
        format!("{}", created.vendor_ref().id().entity_type()),
        "Vendor"
    );
    Ok(())
}
