//! 补齐 serde crate 中 Jackson/JSONB 兼容层与适配器的剩余覆盖率。

use chrono_tz::Tz;
use ddd_4_rust_core::{
    AggregateVersion, EncryptedData, EntityId, EntityIdFactory, EntityIdPath, IntegerEntityId,
    JandexEntityIdFactory,
};
use ddd_4_rust_serde::{
    deserialize_aggregate_version, deserialize_aggregate_version_opt,
    deserialize_entity_id_path_with_factory, serialize_aggregate_version,
    serialize_aggregate_version_opt, serialize_entity_id_path, ZonedDateTimeError,
    ZonedDateTimeValue,
};
use ddd_4_rust_serde::json::jackson::{
    EntityIdJacksonDeserializer, EntityIdJacksonSerializer, EntityIdPathJacksonDeserializer,
};
use ddd_4_rust_serde::json::jsonb::EncryptedDataJsonb;
use ddd_4_rust_serde::json::jackson::EncryptedDataJackson;
#[cfg(feature = "xml")]
use ddd_4_rust_serde::xml::jaxb::EncryptedDataJaxb;
use serde::{Deserialize, Serialize};

/// 测试用实体 ID 工厂，仅识别 `Person` 类型。
struct PersonEntityIdFactory;

impl EntityIdFactory for PersonEntityIdFactory {
    /// 判断工厂是否支持给定实体类型。
    fn contains_type(&self, entity_type: &str) -> bool {
        entity_type == "Person"
    }

    /// 判断给定实体类型与 ID 字符串是否合法。
    fn is_valid(&self, entity_type: &str, id: &str) -> bool {
        self.contains_type(entity_type) && id.parse::<i32>().is_ok()
    }

    /// 根据类型名与 ID 字符串创建实体标识。
    fn create_entity_id(&self, entity_type: &str, id: &str) -> Option<Box<dyn EntityId>> {
        if !self.is_valid(entity_type, id) {
            return None;
        }
        Some(Box::new(
            IntegerEntityId::new(entity_type, id.parse().ok()?).ok()?,
        ))
    }
}

/// 通过 JSONB 公开别名构造加密数据并调用 `EncryptedData` trait getter。
#[test]
fn encrypted_data_trait_getters_via_public_aliases() {
    let jsonb = EncryptedDataJsonb {
        key_id: "key-1".into(),
        key_version: "v1".into(),
        data_type: "text/plain".into(),
        content_type: "application/octet-stream".into(),
        encrypted_data: vec![1, 2, 3],
    };
    assert_eq!(jsonb.key_id(), "key-1");
    assert_eq!(jsonb.key_version(), "v1");
    assert_eq!(jsonb.data_type(), "text/plain");
    assert_eq!(jsonb.content_type(), "application/octet-stream");
    assert_eq!(jsonb.encrypted_data(), &[1, 2, 3]);

    let jackson: EncryptedDataJackson = jsonb.clone();
    assert_eq!(jackson.key_id(), "key-1");
    #[cfg(feature = "xml")]
    {
        let jaxb: EncryptedDataJaxb = jsonb;
        assert_eq!(jaxb.encrypted_data(), &[1, 2, 3]);
    }
}

/// 覆盖 Jackson 实体 ID 序列化/反序列化委托路径。
#[test]
fn entity_id_jackson_serializer_and_deserializer() -> Result<(), Box<dyn std::error::Error>> {
    let entity_id = IntegerEntityId::new("Person", 42)?;
    assert_eq!(
        EntityIdJacksonSerializer::serialize(&entity_id),
        "Person 42"
    );
    let factory = PersonEntityIdFactory;
    let restored = EntityIdJacksonDeserializer::deserialize(&factory, Some("Person 42"))
        .expect("deserialized");
    assert_eq!(restored.as_string(), "42");
    assert!(EntityIdJacksonDeserializer::deserialize(&factory, Some("bad")).is_none());
    Ok(())
}

/// 覆盖 `EntityIdPathJacksonDeserializer` 的 marshal/unmarshal 路径。
#[test]
fn entity_id_path_jackson_marshal_and_unmarshal() -> Result<(), Box<dyn std::error::Error>> {
    let factory = PersonEntityIdFactory;
    let id = IntegerEntityId::new("Person", 7)?;
    let path = EntityIdPath::new(vec![std::sync::Arc::new(id) as std::sync::Arc<dyn EntityId>])?;
    let wire = EntityIdPathJacksonDeserializer::marshal(Some(&path)).expect("marshal");
    let restored = EntityIdPathJacksonDeserializer::unmarshal(
        &factory as &dyn EntityIdFactory,
        Some(&wire),
    )
    .expect("unmarshal");
    assert_eq!(restored.to_string(), path.to_string());
    assert!(EntityIdPathJacksonDeserializer::marshal(None).is_none());
    assert!(
        EntityIdPathJacksonDeserializer::unmarshal(&factory as &dyn EntityIdFactory, Some("bad"))
            .is_none()
    );
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct AggregateVersionHolder {
    #[serde(
        serialize_with = "serialize_aggregate_version",
        deserialize_with = "deserialize_aggregate_version"
    )]
    version: AggregateVersion,
}

#[derive(Serialize, Deserialize)]
struct OptionalAggregateVersionHolder {
    #[serde(
        serialize_with = "serialize_aggregate_version_opt",
        deserialize_with = "deserialize_aggregate_version_opt"
    )]
    version: Option<AggregateVersion>,
}

/// 覆盖聚合版本适配器的 serde 往返与可选分支。
#[test]
fn aggregate_version_adapter_round_trip_and_option_paths() -> Result<(), Box<dyn std::error::Error>>
{
    let holder = AggregateVersionHolder {
        version: AggregateVersion::new(11),
    };
    let json = serde_json::to_string(&holder)?;
    let decoded: AggregateVersionHolder = serde_json::from_str(&json)?;
    assert_eq!(decoded.version.as_u32(), 11);

    let none_holder = OptionalAggregateVersionHolder { version: None };
    assert_eq!(serde_json::to_string(&none_holder)?, r#"{"version":null}"#);
    let some_holder = OptionalAggregateVersionHolder {
        version: Some(AggregateVersion::new(3)),
    };
    let some_json = serde_json::to_string(&some_holder)?;
    let decoded_some: OptionalAggregateVersionHolder = serde_json::from_str(&some_json)?;
    assert_eq!(decoded_some.version.map(|v| v.as_u32()), Some(3));
    Ok(())
}

#[derive(Serialize)]
struct EntityIdPathHolder {
    #[serde(serialize_with = "serialize_entity_id_path")]
    path: EntityIdPath,
}

/// 覆盖实体 ID 路径适配器的序列化与反序列化失败路径。
#[test]
fn entity_id_path_adapter_success_and_invalid_deserialize(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;

    let factory = JandexEntityIdFactory::new()?;
    let id = IntegerEntityId::new("Person", 5)?;
    let path = EntityIdPath::new(vec![Arc::new(id) as Arc<dyn EntityId>])?;
    let json = serde_json::to_string(&EntityIdPathHolder { path: path.clone() })?;
    assert!(json.contains('5'));

    let mut de = serde_json::Deserializer::from_str(r#""Unknown 9""#);
    let invalid = deserialize_entity_id_path_with_factory(
        &factory as &dyn EntityIdFactory,
        &mut de,
    );
    assert!(invalid.is_err());
    Ok(())
}

/// 覆盖 `ZonedDateTimeValue` 的解析、格式化与 serde 错误分支。
#[test]
fn zoned_date_time_value_success_and_error_paths() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = ZonedDateTimeValue::parse("2020-01-15T10:30:00+00:00[UTC]")?;
    assert_eq!(parsed.timezone(), Tz::UTC);
    assert!(parsed.to_java_string().ends_with("[UTC]"));
    assert_eq!(parsed.to_string(), parsed.to_java_string());
    assert_eq!(format!("{parsed}"), parsed.to_java_string());

    assert_eq!(
        ZonedDateTimeValue::parse("no-zone").unwrap_err(),
        ZonedDateTimeError::MissingZone
    );
    assert!(matches!(
        ZonedDateTimeValue::parse("2020-01-01T00:00:00Z[Not/A/Real/Zone]"),
        Err(ZonedDateTimeError::InvalidZone(_))
    ));
    assert!(matches!(
        ZonedDateTimeValue::parse("not-a-date[UTC]"),
        Err(ZonedDateTimeError::InvalidDateTime(_))
    ));

    let round_trip: ZonedDateTimeValue = serde_json::from_str(r#""2020-01-15T10:30:00+00:00[UTC]""#)?;
    assert_eq!(round_trip, parsed);
    assert!(serde_json::from_str::<ZonedDateTimeValue>(r#""broken[UTC]""#).is_err());
    Ok(())
}
