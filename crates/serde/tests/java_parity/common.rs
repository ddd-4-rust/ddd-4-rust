//! `common` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{
    AggregateVersion, DomainEvent, EntityId, EntityIdFactory, EntityIdPath, Event, EventId,
    EventType, IntegerEntityId,
};
use ddd_4_rust_serde::{
    AbstractDomainEvent, AbstractEvent, DddSerdeModule, ZonedDateTimeError, ZonedDateTimeValue,
};
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use std::sync::Arc;
use uuid::Uuid;

/// `TestEntityIdFactory` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct TestEntityIdFactory;

impl EntityIdFactory for TestEntityIdFactory {
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn contains_type(&self, entity_type: &str) -> bool {
        entity_type == "Person"
    }

    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn is_valid(&self, entity_type: &str, id: &str) -> bool {
        self.contains_type(entity_type) && id.parse::<i32>().is_ok()
    }

    /// 执行 `create_entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create_entity_id(&self, entity_type: &str, id: &str) -> Option<Box<dyn EntityId>> {
        if !self.is_valid(entity_type, id) {
            return None;
        }
        Some(Box::new(
            IntegerEntityId::new(entity_type, id.parse().ok()?).ok()?,
        ))
    }
}

#[derive(Serialize, Deserialize)]
/// `RequiredVersionWire` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct RequiredVersionWire {
    #[serde(
        serialize_with = "ddd_4_rust_serde::serialize_aggregate_version",
        deserialize_with = "ddd_4_rust_serde::deserialize_aggregate_version"
    )]
    version: AggregateVersion,
}

#[derive(Serialize, Deserialize)]
/// `OptionalVersionWire` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct OptionalVersionWire {
    #[serde(
        serialize_with = "ddd_4_rust_serde::serialize_aggregate_version_opt",
        deserialize_with = "ddd_4_rust_serde::deserialize_aggregate_version_opt"
    )]
    version: Option<AggregateVersion>,
}

#[expect(
    clippy::too_many_lines,
    reason = "one golden compatibility matrix keeps all three Java serializer namespaces aligned"
)]
/// 执行 `exercise_shared_adapters` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn exercise_shared_adapters(
    event_id: &EventId,
    entity_id: &IntegerEntityId,
    path: &EntityIdPath,
) -> Result<(), Box<dyn std::error::Error>> {
    use ddd_4_rust_serde::json::{jsonb, serde as serde_api};

    assert_eq!(
        serde_json::to_string(&RequiredVersionWire {
            version: AggregateVersion::new(7),
        })?,
        r#"{"version":7}"#
    );
    assert_eq!(
        serde_json::from_str::<RequiredVersionWire>(r#"{"version":7}"#)?.version,
        AggregateVersion::new(7)
    );
    assert_eq!(
        serde_json::to_string(&OptionalVersionWire { version: None })?,
        r#"{"version":null}"#
    );
    assert!(
        serde_json::from_str::<OptionalVersionWire>(r#"{"version":null}"#)?
            .version
            .is_none()
    );
    assert_eq!(
        ddd_4_rust_serde::serialize_entity_id(entity_id, serde_json::value::Serializer)?,
        serde_json::Value::String("Person 17".to_owned())
    );
    assert_eq!(
        ddd_4_rust_serde::serialize_entity_id_opt(
            &Some(entity_id as &dyn EntityId),
            serde_json::value::Serializer,
        )?,
        serde_json::Value::String("Person 17".to_owned())
    );
    assert_eq!(
        ddd_4_rust_serde::serialize_entity_id_opt(&None, serde_json::value::Serializer)?,
        serde_json::Value::Null
    );
    let raw_id = ddd_4_rust_serde::deserialize_entity_id_string(
        serde_json::Value::String("Person 17".to_owned()).into_deserializer(),
    )?;
    assert_eq!(raw_id, "Person 17");
    assert_eq!(
        ddd_4_rust_serde::serialize_entity_id_path(path, serde_json::value::Serializer)?,
        serde_json::Value::String("Person 17".to_owned())
    );
    let decoded_path = ddd_4_rust_serde::deserialize_entity_id_path_with_factory(
        &TestEntityIdFactory,
        serde_json::Value::String("Person 17".to_owned()).into_deserializer(),
    )?;
    assert_eq!(decoded_path.to_string(), "Person 17");

    assert_eq!(
        serde_api::AggregateVersionAdapter::serialize(&AggregateVersion::new(7)),
        7
    );
    assert_eq!(
        serde_api::AggregateVersionAdapter::deserialize(7),
        AggregateVersion::new(7)
    );
    assert_eq!(
        serde_api::EntityIdAdapter::serialize(entity_id),
        "Person 17"
    );
    assert!(
        serde_api::EntityIdAdapter::deserialize(&TestEntityIdFactory, Some("Person 17")).is_some()
    );
    assert!(serde_api::EntityIdAdapter::deserialize(&TestEntityIdFactory, None).is_none());
    assert_eq!(
        serde_api::EntityIdPathAdapter::serialize(Some(path)).as_deref(),
        Some("Person 17")
    );
    assert!(
        serde_api::EntityIdPathAdapter::deserialize(&TestEntityIdFactory, Some("Person 17"))
            .is_some()
    );

    assert_eq!(
        jsonb::AggregateVersionJsonbAdapter::marshal(Some(&AggregateVersion::new(7))),
        Some(7)
    );
    assert_eq!(
        jsonb::AggregateVersionJsonbAdapter::unmarshal(Some(7)),
        Some(AggregateVersion::new(7))
    );
    assert_eq!(
        jsonb::EntityIdJsonbAdapter::marshal(Some(entity_id)),
        Some("Person 17".to_owned())
    );
    assert!(
        jsonb::EntityIdJsonbAdapter::unmarshal(&TestEntityIdFactory, Some("Person 17")).is_some()
    );
    assert_eq!(
        jsonb::EntityIdPathJsonbAdapter::marshal(Some(path)).as_deref(),
        Some("Person 17")
    );
    assert!(
        jsonb::EntityIdPathJsonbAdapter::unmarshal(&TestEntityIdFactory, Some("Person 17"))
            .is_some()
    );
    assert_eq!(
        jsonb::EventIdJsonbAdapter::marshal(Some(event_id)).as_deref(),
        Some(event_id.as_string().as_str())
    );
    assert_eq!(
        jsonb::EventIdJsonbAdapter::unmarshal(Some(&event_id.as_string())),
        Some(event_id.clone())
    );

    #[cfg(feature = "xml")]
    {
        use ddd_4_rust_serde::xml::jaxb;
        assert_eq!(
            jaxb::AggregateVersionXmlAdapter::marshal(Some(&AggregateVersion::new(7))),
            Some(7)
        );
        assert_eq!(
            jaxb::AggregateVersionXmlAdapter::unmarshal(Some(7)),
            Some(AggregateVersion::new(7))
        );
        assert_eq!(
            jaxb::EntityIdXmlAdapter::marshal(Some(entity_id)),
            Some("Person 17".to_owned())
        );
        assert!(
            jaxb::EntityIdXmlAdapter::unmarshal(&TestEntityIdFactory, Some("Person 17")).is_some()
        );
        assert_eq!(
            jaxb::EntityIdPathXmlAdapter::marshal(Some(path)).as_deref(),
            Some("Person 17")
        );
        assert!(
            jaxb::EntityIdPathXmlAdapter::unmarshal(&TestEntityIdFactory, Some("Person 17"))
                .is_some()
        );
        assert_eq!(
            jaxb::EventIdXmlAdapter::marshal(Some(event_id)).as_deref(),
            Some(event_id.as_string().as_str())
        );
        assert_eq!(
            jaxb::EventIdXmlAdapter::unmarshal(Some(&event_id.as_string())),
            Some(event_id.clone())
        );
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "the Java parity scenario intentionally verifies one complete event wire contract"
)]
/// 执行 `run_scenario` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn run_scenario(
    namespace: &str,
    java_type: &str,
    scenario: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(!java_type.is_empty());
    assert!(matches!(namespace, "jackson" | "jsonb" | "jaxb"));

    let source = if scenario & 1 == 0 {
        "2020-07-15T14:00:00+02:00[Europe/Berlin]"
    } else {
        "2020-01-15T13:00:00+01:00[Europe/Berlin]"
    };
    let zoned = ZonedDateTimeValue::parse(source)?;
    assert_eq!(zoned.timezone().name(), "Europe/Berlin");
    assert_eq!(zoned.to_java_string(), source);
    let json = serde_json::to_string(&zoned)?;
    assert_eq!(serde_json::from_str::<ZonedDateTimeValue>(&json)?, zoned);

    let event_id = EventId::from_uuid(Uuid::parse_str("bb05f34d-4eac-4f6a-b3c2-5c89269720f3")?);
    let event = AbstractEvent::new_zoned(
        event_id.clone(),
        EventType::new(java_type)?,
        *zoned.instant(),
        zoned.timezone(),
        Some(event_id.clone()),
        Some(event_id.clone()),
    );
    assert_eq!(event.zoned_event_timestamp(), &zoned);
    assert_eq!(event.event_id(), &event_id);
    assert_eq!(event.event_type().as_str(), java_type);
    assert_eq!(event.event_timestamp(), zoned.instant());
    assert_eq!(event.correlation_id(), Some(&event_id));
    assert_eq!(event.causation_id(), Some(&event_id));
    assert_eq!(Event::event_id(&event), &event_id);
    assert_eq!(Event::event_type(&event).as_str(), java_type);
    assert_eq!(Event::event_timestamp(&event), zoned.instant());
    assert_eq!(Event::correlation_id(&event), Some(&event_id));
    assert_eq!(Event::causation_id(&event), Some(&event_id));
    let decoded_event: AbstractEvent = serde_json::from_str(&serde_json::to_string(&event)?)?;
    assert_eq!(decoded_event.zoned_event_timestamp(), &zoned);
    let utc_event = AbstractEvent::new(
        event_id.clone(),
        EventType::new(java_type)?,
        *zoned.instant(),
        None,
        None,
    );
    assert_eq!(utc_event.zoned_event_timestamp().timezone(), chrono_tz::UTC);
    assert!(
        AbstractEvent::new_now(EventType::new(java_type)?)
            .correlation_id()
            .is_none()
    );

    let entity_id = IntegerEntityId::new("Person", 17)?;
    let entity_id_path = EntityIdPath::new(vec![Arc::new(entity_id.clone())])?;
    let domain_event = AbstractDomainEvent::new_zoned(
        event_id.clone(),
        EventType::new(java_type)?,
        *zoned.instant(),
        zoned.timezone(),
        Some(event_id.clone()),
        Some(event_id.clone()),
        entity_id_path.clone(),
        Some(AggregateVersion::new(7)),
    );
    assert_eq!(domain_event.event_id(), &event_id);
    assert_eq!(domain_event.event_type().as_str(), java_type);
    assert_eq!(domain_event.event_timestamp(), zoned.instant());
    assert_eq!(domain_event.correlation_id(), Some(&event_id));
    assert_eq!(domain_event.causation_id(), Some(&event_id));
    assert_eq!(domain_event.entity_id_path().to_string(), "Person 17");
    assert_eq!(domain_event.entity_id().as_typed_string(), "Person 17");
    assert_eq!(
        domain_event.aggregate_version(),
        Some(&AggregateVersion::new(7))
    );
    assert_eq!(domain_event.zoned_event_timestamp(), &zoned);
    assert_eq!(Event::event_id(&domain_event), &event_id);
    assert_eq!(Event::event_type(&domain_event).as_str(), java_type);
    assert_eq!(Event::event_timestamp(&domain_event), zoned.instant());
    assert_eq!(Event::correlation_id(&domain_event), Some(&event_id));
    assert_eq!(Event::causation_id(&domain_event), Some(&event_id));
    assert_eq!(
        DomainEvent::entity_id_path(&domain_event).to_string(),
        entity_id_path.to_string()
    );
    assert_eq!(
        DomainEvent::aggregate_version(&domain_event),
        Some(&AggregateVersion::new(7))
    );
    let utc_domain_event = AbstractDomainEvent::new(
        event_id.clone(),
        EventType::new(java_type)?,
        *zoned.instant(),
        None,
        None,
        entity_id_path.clone(),
        None,
    );
    assert_eq!(
        utc_domain_event.zoned_event_timestamp().timezone(),
        chrono_tz::UTC
    );
    assert!(
        AbstractDomainEvent::new_now(EventType::new(java_type)?, entity_id_path.clone(), None,)
            .aggregate_version()
            .is_none()
    );

    let _module = DddSerdeModule::new();
    #[expect(
        clippy::default_constructed_unit_structs,
        reason = "explicitly exercises the public Default compatibility entry point"
    )]
    let default_module = DddSerdeModule::default();
    assert_eq!(std::mem::size_of_val(&default_module), 0);
    assert!(matches!(
        ZonedDateTimeValue::parse("2020-01-01T00:00:00Z"),
        Err(ZonedDateTimeError::MissingZone)
    ));
    assert!(matches!(
        ZonedDateTimeValue::parse("2020-01-01T00:00:00Z[No/Such_Zone]"),
        Err(ZonedDateTimeError::InvalidZone(_))
    ));
    assert!(matches!(
        ZonedDateTimeValue::parse("invalid[Europe/Berlin]"),
        Err(ZonedDateTimeError::InvalidDateTime(_))
    ));
    exercise_shared_adapters(&event_id, &entity_id, &entity_id_path)?;
    assert_eq!(AggregateVersion::new(7).as_u32(), 7);

    #[cfg(feature = "xml")]
    if namespace == "jaxb" {
        #[derive(serde::Serialize)]
        #[serde(rename = "timestamp")]
        /// `Timestamp` 表示与同名 Java 类型对应的 Rust 领域对象。
        /// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
        struct Timestamp<'a> {
            #[serde(rename = "$text")]
            value: &'a ZonedDateTimeValue,
        }
        let xml = quick_xml::se::to_string(&Timestamp { value: &zoned })?;
        assert_eq!(xml, format!("<timestamp>{source}</timestamp>"));
    }
    Ok(())
}
