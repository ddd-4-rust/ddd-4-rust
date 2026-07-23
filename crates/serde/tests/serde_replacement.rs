//! `serde_replacement` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Verifies the Serde-native API and the frozen Java-name compatibility facade.

use ddd_4_rust_core::{AggregateVersion, EntityId, EntityIdFactory, IntegerEntityId};
use ddd_4_rust_serde::json::serde::{AggregateVersionAdapter, EntityIdAdapter};

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

#[test]
/// 执行 `serde_is_the_primary_json_adapter_api` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn serde_is_the_primary_json_adapter_api() -> Result<(), Box<dyn std::error::Error>> {
    let version = AggregateVersion::new(7);
    assert_eq!(AggregateVersionAdapter::serialize(&version), 7);
    assert_eq!(AggregateVersionAdapter::deserialize(7), version);

    let entity_id = IntegerEntityId::new("Person", 17)?;
    assert_eq!(EntityIdAdapter::serialize(&entity_id), "Person 17");
    assert!(EntityIdAdapter::deserialize(&TestEntityIdFactory, Some("Person 17")).is_some());
    Ok(())
}

#[test]
#[expect(
    deprecated,
    reason = "the frozen Java namespace is intentionally exercised as a compatibility facade"
)]
/// 执行 `legacy_jackson_names_delegate_to_serde` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn legacy_jackson_names_delegate_to_serde() {
    use ddd_4_rust_serde::json::jackson::{
        AggregateVersionJacksonDeserializer, AggregateVersionJacksonSerializer,
    };

    let version = AggregateVersion::new(9);
    assert_eq!(
        AggregateVersionJacksonSerializer::serialize(&version),
        AggregateVersionAdapter::serialize(&version)
    );
    assert_eq!(
        AggregateVersionJacksonDeserializer::deserialize(9),
        AggregateVersionAdapter::deserialize(9)
    );
}
