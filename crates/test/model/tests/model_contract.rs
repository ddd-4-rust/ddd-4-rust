//! `model_contract` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Executable coverage for the complete non-published Java compatibility model.
use ddd_4_rust_core::{AggregateRoot, AggregateVersion, DomainEvent, Entity, EntityId, Event};
use ddd_4_rust_test_model::{
    DuplicateVendorKeyException, PersonCreatedEvent, PersonId, PersonIdJsonbAdapter, PersonName,
    PersonNameChangedEvent, PersonNameJsonbAdapter, PersonNotFoundException, Vendor,
    VendorCreatedEvent, VendorEventId, VendorId, VendorIdJsonbAdapter, VendorKey,
    VendorKeyJsonbAdapter, VendorName, VendorNameJsonbAdapter, VendorRef,
};
use uuid::Uuid;

/// 执行 `vendor_ref` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn vendor_ref() -> Result<VendorRef, Box<dyn std::error::Error>> {
    Ok(VendorRef::new(
        VendorId::from_uuid(Uuid::nil()),
        VendorKey::new("V00001")?,
        VendorName::new("Acme")?,
    ))
}

#[test]
/// 执行 `value_objects_and_jsonb_adapters_cover_wire_boundaries` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn value_objects_and_jsonb_adapters_cover_wire_boundaries() -> Result<(), Box<dyn std::error::Error>>
{
    let vendor_id = VendorId::from_uuid(Uuid::nil());
    assert_eq!(vendor_id.as_uuid(), &Uuid::nil());
    assert_eq!(
        VendorId::value_of(Some(&vendor_id.to_string())),
        Some(vendor_id)
    );
    assert!(VendorId::value_of(Some("invalid")).is_none());
    assert!(VendorId::value_of(None).is_none());
    assert_eq!(vendor_id.entity_type().as_string(), "Vendor");
    assert_eq!(vendor_id.as_string(), Uuid::nil().to_string());
    assert_ne!(VendorId::new(), VendorId::default());

    let person_id = PersonId::new(17);
    assert_eq!(person_id.as_i32(), 17);
    assert_eq!(person_id.to_string(), "17");
    assert_eq!(person_id.as_string(), "17");
    assert_eq!(person_id.entity_type().as_string(), "Person");
    assert!(PersonId::is_valid(None));
    assert!(PersonId::is_valid(Some("17")));
    assert!(!PersonId::is_valid(Some("x")));
    assert_eq!(PersonId::value_of(Some("17")), Some(person_id));
    assert!(PersonId::value_of(None).is_none());

    let key = VendorKey::new("V00001")?;
    assert_eq!(key.as_str(), "V00001");
    assert_eq!(key.to_string(), "V00001");
    assert!(VendorKey::is_valid(None));
    assert!(VendorKey::is_valid(Some("V99999")));
    assert!(!VendorKey::is_valid(Some("bad")));
    assert!(VendorKey::new("bad").is_err());
    assert_eq!(VendorKey::value_of(Some("V00001"))?, Some(key.clone()));
    assert_eq!(VendorKey::value_of(None)?, None);

    let vendor_name = VendorName::new("Acme")?;
    assert_eq!(vendor_name.as_str(), "Acme");
    assert_eq!(vendor_name.to_string(), "Acme");
    assert!(VendorName::is_valid(None));
    assert!(!VendorName::is_valid(Some("")));
    assert!(VendorName::new("").is_err());
    assert_eq!(
        VendorName::value_of(Some("Acme"))?,
        Some(vendor_name.clone())
    );
    assert_eq!(VendorName::value_of(None)?, None);

    let person_name = PersonName::new("Peter")?;
    assert_eq!(person_name.as_str(), "Peter");
    assert_eq!(person_name.to_string(), "Peter");
    assert!(PersonName::is_valid(None));
    assert!(!PersonName::is_valid(Some("")));
    assert!(PersonName::new("").is_err());
    assert_eq!(
        PersonName::value_of(Some("Peter"))?,
        Some(person_name.clone())
    );
    assert_eq!(PersonName::value_of(None)?, None);

    assert_eq!(
        PersonIdJsonbAdapter::adapt_to_json(Some(&person_id)),
        Some(17)
    );
    assert_eq!(
        PersonIdJsonbAdapter::adapt_from_json(Some(17)),
        Some(person_id)
    );
    assert_eq!(PersonIdJsonbAdapter::adapt_to_json(None), None);
    assert_eq!(PersonIdJsonbAdapter::adapt_from_json(None), None);
    assert_eq!(
        VendorIdJsonbAdapter::adapt_to_json(Some(&vendor_id)),
        Some(Uuid::nil().to_string())
    );
    assert_eq!(
        VendorIdJsonbAdapter::adapt_from_json(Some(&Uuid::nil().to_string())),
        Some(vendor_id)
    );
    assert_eq!(
        VendorKeyJsonbAdapter::adapt_to_json(Some(&key)),
        Some("V00001")
    );
    assert_eq!(
        VendorKeyJsonbAdapter::adapt_from_json(Some("V00001"))?,
        Some(key)
    );
    assert_eq!(
        VendorNameJsonbAdapter::adapt_to_json(Some(&vendor_name)),
        Some("Acme")
    );
    assert_eq!(
        VendorNameJsonbAdapter::adapt_from_json(Some("Acme"))?,
        Some(vendor_name)
    );
    assert_eq!(
        PersonNameJsonbAdapter::adapt_to_json(Some(&person_name)),
        Some("Peter")
    );
    assert_eq!(
        PersonNameJsonbAdapter::adapt_from_json(Some("Peter"))?,
        Some(person_name)
    );
    Ok(())
}

#[test]
/// 执行 `events_expose_typed_and_erased_domain_contracts` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn events_expose_typed_and_erased_domain_contracts() -> Result<(), Box<dyn std::error::Error>> {
    let reference = vendor_ref()?;
    assert_eq!(reference.id(), &VendorId::from_uuid(Uuid::nil()));
    assert_eq!(reference.key().as_str(), "V00001");
    assert_eq!(reference.name().as_str(), "Acme");
    assert_eq!(reference.to_string(), "Vendor V00001");

    let vendor_event = VendorCreatedEvent::new(reference.clone())?;
    assert_eq!(vendor_event.vendor_ref(), &reference);
    assert_eq!(vendor_event.event_type().as_str(), "VendorCreatedEvent");
    assert!(vendor_event.correlation_id().is_none());
    assert!(vendor_event.causation_id().is_none());
    assert_eq!(
        DomainEvent::<VendorId>::entity_id(&vendor_event),
        reference.id()
    );
    assert_eq!(
        DomainEvent::<VendorId>::entity_id_path(&vendor_event).size(),
        1
    );
    assert_eq!(
        DomainEvent::<VendorId>::aggregate_version(&vendor_event),
        Some(&AggregateVersion::new(0))
    );
    assert!(vendor_event.to_string().contains("Created vendor"));

    let created = PersonCreatedEvent::builder()
        .vendor_ref(reference.clone())
        .person_id(PersonId::new(1))
        .person_name(PersonName::new("Peter")?)
        .aggregate_version(AggregateVersion::new(1))
        .build()?;
    assert_eq!(created.vendor_ref(), &reference);
    assert_eq!(created.person_id(), PersonId::new(1));
    assert_eq!(created.person_name().as_str(), "Peter");
    assert_eq!(created.event_type().as_str(), "PersonCreatedEvent");
    assert_eq!(DomainEvent::<PersonId>::entity_id_path(&created).size(), 1);
    assert_eq!(
        DomainEvent::<PersonId>::entity_id(&created),
        &PersonId::new(1)
    );
    assert_eq!(
        DomainEvent::<PersonId>::aggregate_version(&created),
        Some(&AggregateVersion::new(1))
    );
    assert_eq!(
        DomainEvent::<dyn EntityId>::entity_id(&created).as_typed_string(),
        "Person 1"
    );
    assert!(created.to_string().contains("Created person #1"));
    assert!(PersonCreatedEvent::builder().build().is_err());

    let changed = PersonNameChangedEvent::new(
        reference.clone(),
        PersonId::new(1),
        PersonName::new("Peter")?,
        PersonName::new("Harry")?,
        AggregateVersion::new(2),
    )?;
    assert_eq!(changed.vendor_ref(), &reference);
    assert_eq!(changed.person_id(), PersonId::new(1));
    assert_eq!(changed.old_name().as_str(), "Peter");
    assert_eq!(changed.new_name().as_str(), "Harry");
    assert_eq!(DomainEvent::<PersonId>::entity_id_path(&changed).size(), 2);
    assert_eq!(
        DomainEvent::<PersonId>::aggregate_version(&changed),
        Some(&AggregateVersion::new(2))
    );
    assert_eq!(
        DomainEvent::<dyn EntityId>::entity_id(&changed).as_typed_string(),
        "Person 1"
    );
    assert!(changed.to_string().contains("Peter' to 'Harry"));

    let event_id = VendorEventId::new(reference.id(), 3);
    assert_eq!(event_id.vendor_id(), Uuid::nil().to_string());
    assert_eq!(event_id.event_number(), 3);
    assert!(event_id.to_string().ends_with("-3"));
    Ok(())
}

#[test]
/// 执行 `aggregate_and_exception_failure_paths_are_structured` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn aggregate_and_exception_failure_paths_are_structured() -> Result<(), Box<dyn std::error::Error>>
{
    let reference = vendor_ref()?;
    let duplicate = DuplicateVendorKeyException::new(reference.key().clone());
    assert_eq!(duplicate.key(), reference.key());
    assert!(duplicate.to_string().contains("already exists"));
    let missing = PersonNotFoundException::new(reference.clone(), PersonId::new(99));
    assert_eq!(missing.vendor_ref(), &reference);
    assert_eq!(missing.person_id(), PersonId::new(99));
    assert!(missing.to_string().contains("not found"));

    let mut rejecting = |key: &VendorKey| Err(DuplicateVendorKeyException::new(key.clone()));
    assert!(
        Vendor::new(
            *reference.id(),
            reference.key().clone(),
            reference.name().clone(),
            &mut rejecting,
        )
        .is_err()
    );

    let mut accepting = |_: &VendorKey| Ok(());
    let mut vendor = Vendor::new(
        *reference.id(),
        reference.key().clone(),
        reference.name().clone(),
        &mut accepting,
    )?;
    assert_eq!(vendor.vendor_ref(), &reference);
    assert_eq!(vendor.id(), reference.id());
    assert_eq!(vendor.entity_type().as_string(), "Vendor");
    assert_eq!(vendor.aggregate_version(), AggregateVersion::new(1));
    assert!(
        vendor
            .change_person_name(PersonId::new(99), PersonName::new("Nobody")?)
            .is_err()
    );
    vendor.mark_changes_as_committed();
    let person_id = vendor.add_person(PersonName::new("Peter")?)?;
    assert_eq!(vendor.persons()[0].id(), person_id);
    vendor.mark_changes_as_committed();
    vendor.change_person_name(person_id, PersonName::new("Harry")?)?;
    let history_event = VendorCreatedEvent::new(reference.clone())?;
    let history: Vec<Box<dyn DomainEvent<dyn EntityId>>> = vec![Box::new(history_event)];
    vendor.load_from_history(&history)?;
    vendor.apply(Box::new(VendorCreatedEvent::new(reference)?))?;
    assert!(!vendor.uncommitted_changes().is_empty());
    Ok(())
}
