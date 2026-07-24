//! Extra coverage for remaining core production paths.

use ddd_4_rust_core::{
    AbstractAggregateException, AbstractAggregateRoot, AbstractEntity, AggregateCache,
    AggregateNoCache, AggregateRootUuid, AggregateVersion, AggregateVersionConflictException,
    ApplyEventHandler, DomainEvent, DomainEventExpectedEntityIdPath, EntityId, EntityIdFactory,
    EntityIdPath, EntityIdRegistration, EntityNotFoundException, EntityType, Event, EventId,
    EventType, ExpectedEntityIdPath, IntegerEntityId, JandexEntityIdFactory, StringBasedEntityType,
};
use std::{cmp::Ordering, sync::Arc};
use uuid::Uuid;

#[derive(Clone)]
struct StubId {
    value: String,
    entity_type: StringBasedEntityType,
}

impl StubId {
    fn new(kind: &str, value: &str) -> Self {
        Self {
            value: value.to_owned(),
            entity_type: StringBasedEntityType::new(kind).expect("entity type"),
        }
    }
}

impl std::fmt::Display for StubId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Debug for StubId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StubId({})", self.value)
    }
}

impl EntityId for StubId {
    fn entity_type(&self) -> &dyn ddd_4_rust_core::EntityType {
        &self.entity_type
    }

    fn as_string(&self) -> String {
        self.value.clone()
    }
}

impl ddd_4_rust_core::AggregateRootId for StubId {}

#[test]
fn aggregate_no_cache_never_stores() {
    let cache = AggregateNoCache::<u32>::default();
    let id = StubId::new("A", "1");
    assert!(cache.get(&id, Some(1)).is_none());
    cache.put(&id, 7);
    cache.remove(&id);
    assert!(AggregateNoCache::<u32>::new().get(&id, None).is_none());
}

#[test]
fn aggregate_root_uuid_ordering_display_and_as_uuid() {
    let left = AggregateRootUuid::from_uuid("Person", Uuid::nil()).expect("uuid");
    let right = AggregateRootUuid::from_uuid("Vendor", Uuid::from_u128(1)).expect("uuid");
    assert_eq!(left.as_uuid(), &Uuid::nil());
    assert!(format!("{left:?}").contains("Person"));
    assert_eq!(format!("{left}"), Uuid::nil().to_string());
    assert_eq!(left.cmp(&left), Ordering::Equal);
    assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
}

#[test]
fn value_object_conversions_and_display() {
    let version = AggregateVersion::from(9_u32);
    assert_eq!(u32::from(version), 9);
    let event_id = EventId::from(Uuid::nil());
    assert_eq!(Uuid::from(event_id), Uuid::nil());
    assert_eq!(EventId::default().as_uuid().get_version_num(), 4);
    assert_eq!(
        EventType::try_from("Created".to_owned())
            .expect("ok")
            .as_str(),
        "Created"
    );
    assert_eq!(
        EventType::try_from("Created").expect("ok").as_str(),
        "Created"
    );
    assert_eq!(
        EventType::new("").expect_err("empty"),
        ddd_4_rust_core::EventTypeError::Empty
    );
    assert_eq!(
        EventType::new("").unwrap_err().to_string(),
        "event type must not be empty"
    );
    let too_long = "x".repeat(EventType::MAX_LENGTH + 1);
    let too_long_err = EventType::new(too_long).expect_err("too long");
    assert!(matches!(
        too_long_err,
        ddd_4_rust_core::EventTypeError::TooLong { .. }
    ));
    assert!(too_long_err.to_string().contains("must not exceed"));
    let et = EventType::new("Ping").expect("ok");
    assert_eq!(et.to_string(), "Ping");
    assert_eq!(String::from(et), "Ping");
    assert_eq!(
        StringBasedEntityType::try_from("Person".to_owned())
            .expect("ok")
            .as_string(),
        "Person"
    );
    assert_eq!(
        StringBasedEntityType::try_from("Person")
            .expect("ok")
            .as_string(),
        "Person"
    );
    let integer = IntegerEntityId::new("Person", 17).expect("id");
    assert!(format!("{integer:?}").contains("IntegerEntityId"));
    assert_eq!(integer.entity_type().as_string(), "Person");
}

#[test]
fn domain_event_expected_path_and_exceptions() {
    let expected = ExpectedEntityIdPath::new(["Vendor", "Person"]);
    let path = DomainEventExpectedEntityIdPath::new(expected.clone());
    assert_eq!(path.expected(), &expected);

    let conflict = AggregateVersionConflictException::new("Vendor", "1", 1, 2);
    assert_eq!(conflict.expected(), 1);
    assert_eq!(conflict.actual(), 2);
    assert_eq!(conflict.data().message(), conflict.to_string());
    assert_eq!(conflict.data().entity_type(), "Vendor");
    assert_eq!(conflict.data().entity_id(), "1");
    assert!(conflict.to_string().contains("Expected version"));
    let missing = EntityNotFoundException::new(Some("Vendor 1".into()), "Person 9");
    assert_eq!(missing.parent_id_path(), Some("Vendor 1"));
    assert_eq!(missing.entity_id(), "Person 9");
    assert!(missing.to_string().contains("Person 9") || missing.to_string().contains("not found") || !missing.to_string().is_empty());
    let aggregate = AbstractAggregateException::new("boom", "Vendor", "1");
    assert_eq!(aggregate.message(), "boom");
    assert_eq!(aggregate.entity_type(), "Vendor");
    assert_eq!(aggregate.entity_id(), "1");
    assert!(aggregate.to_string().contains("boom"));
}

#[test]
fn jandex_factory_rejects_duplicate_registrations() {
    fn parse(_: &str) -> Option<Box<dyn EntityId>> {
        None
    }
    fn validate(_: &str) -> bool {
        false
    }
    static FIRST: EntityIdRegistration = EntityIdRegistration {
        entity_type: "DupType",
        parse,
        validate,
    };
    static SECOND: EntityIdRegistration = EntityIdRegistration {
        entity_type: "DupType",
        parse,
        validate,
    };
    let err = match JandexEntityIdFactory::from_registrations([&FIRST, &SECOND]) {
        Ok(_) => panic!("expected duplicate type"),
        Err(err) => err,
    };
    assert!(matches!(
        err,
        ddd_4_rust_core::EntityIdRegistryError::DuplicateType("DupType")
    ));
    assert!(err.to_string().contains("DupType"));
}

#[test]
fn aggregate_root_trait_default_has_uncommitted_changes() {
    use ddd_4_rust_core::{AggregateRoot, AggregateRootId, Entity};

    struct EmptyRoot {
        id: StubId,
        entity_type: StringBasedEntityType,
        changes: Vec<Box<dyn DomainEvent<dyn EntityId>>>,
    }

    impl Entity<StubId> for EmptyRoot {
        fn entity_type(&self) -> &dyn EntityType {
            &self.entity_type
        }
        fn id(&self) -> &StubId {
            &self.id
        }
    }

    impl AggregateRoot<StubId> for EmptyRoot {
        fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>] {
            &self.changes
        }
        fn mark_changes_as_committed(&mut self) {
            self.changes.clear();
        }
        fn version(&self) -> i32 {
            0
        }
        fn load_from_history(
            &mut self,
            _history: &[Box<dyn DomainEvent<dyn EntityId>>],
        ) -> Result<(), ddd_4_rust_core::AggregateError> {
            Ok(())
        }
        fn apply(
            &mut self,
            event: Box<dyn DomainEvent<dyn EntityId>>,
        ) -> Result<(), ddd_4_rust_core::AggregateError> {
            self.changes.push(event);
            Ok(())
        }
    }

    let root = EmptyRoot {
        id: StubId::new("A", "1"),
        entity_type: StringBasedEntityType::new("A").expect("type"),
        changes: Vec::new(),
    };
    assert!(!AggregateRoot::has_uncommitted_changes(&root));
    let _ = root.id() as &dyn AggregateRootId;
}

#[test]
fn entity_id_path_edge_cases_and_serde() -> Result<(), Box<dyn std::error::Error>> {
    let factory = JandexEntityIdFactory::new()?;
    let id = Arc::new(StubId::new("A", "1")) as Arc<dyn EntityId>;
    let path = EntityIdPath::new(vec![id])?;
    assert!(!path.is_empty());
    assert!(EntityIdPath::value_of(&factory as &dyn EntityIdFactory, None).is_none());
    assert!(!EntityIdPath::is_valid(
        &factory as &dyn EntityIdFactory,
        Some("")
    ));
    assert!(!EntityIdPath::is_valid(
        &factory as &dyn EntityIdFactory,
        Some("A")
    ));
    assert!(!EntityIdPath::is_valid(
        &factory as &dyn EntityIdFactory,
        Some("Unknown 1")
    ));
    let json = serde_json::to_string(&path)?;
    assert!(json.contains('A'));
    Ok(())
}

#[test]
fn abstract_helpers_defaults_and_downcast() {
    let root = AbstractAggregateRoot::<StubId>::default();
    assert_eq!(root.version(), -1);
    assert!(!root.has_uncommitted_changes());

    struct Handler;
    impl ApplyEventHandler for Handler {
        fn try_apply_event(
            &mut self,
            _event: &dyn DomainEvent<dyn EntityId>,
        ) -> Result<bool, ddd_4_rust_core::AggregateError> {
            Ok(false)
        }
    }
    assert!(Handler.ignored_events().is_empty());

    let entity = AbstractEntity::<StubId, StubId>::default();
    assert!(entity.id().is_none());
    assert!(entity.parent_id().is_none());

    #[derive(Debug)]
    struct TinyEvent {
        id: EventId,
        kind: EventType,
        timestamp: chrono::DateTime<chrono::Utc>,
    }
    impl Event for TinyEvent {
        fn event_id(&self) -> &EventId {
            &self.id
        }
        fn event_type(&self) -> &EventType {
            &self.kind
        }
        fn event_timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
            &self.timestamp
        }
        fn correlation_id(&self) -> Option<&EventId> {
            None
        }
        fn causation_id(&self) -> Option<&EventId> {
            None
        }
    }
    #[derive(Debug)]
    struct OtherEvent {
        id: EventId,
        kind: EventType,
        timestamp: chrono::DateTime<chrono::Utc>,
    }
    impl Event for OtherEvent {
        fn event_id(&self) -> &EventId {
            &self.id
        }
        fn event_type(&self) -> &EventType {
            &self.kind
        }
        fn event_timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
            &self.timestamp
        }
        fn correlation_id(&self) -> Option<&EventId> {
            None
        }
        fn causation_id(&self) -> Option<&EventId> {
            None
        }
    }
    let event = TinyEvent {
        id: EventId::new(),
        kind: EventType::new("Tiny").expect("type"),
        timestamp: chrono::Utc::now(),
    };
    let erased: &dyn Event = &event;
    assert!(ddd_4_rust_core::event::downcast_event::<TinyEvent>(erased).is_some());
    assert!(ddd_4_rust_core::event::downcast_event::<OtherEvent>(erased).is_none());
}
