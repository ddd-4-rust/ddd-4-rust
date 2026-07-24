//! `jandex_entity_id_factory` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.JandexEntityIdFactory`.

use crate::{EntityId, EntityIdFactory};
use std::collections::BTreeMap;
use thiserror::Error;

/// Link-time registration replacing Java/Jandex classpath scanning.
///
/// `EntityIdRegistration` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityIdRegistration {
    /// Stable entity type name.
    ///
    /// 保存 `entity_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_type: &'static str,
    /// Parses the base identifier.
    ///
    /// 保存 `parse` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub parse: fn(&str) -> Option<Box<dyn EntityId>>,
    /// Validates the base identifier.
    ///
    /// 保存 `validate` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub validate: fn(&str) -> bool,
}

inventory::collect!(EntityIdRegistration);

/// Entity identifier registry construction failure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// `EntityIdRegistryError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum EntityIdRegistryError {
    /// Two registrations claimed the same type.
    #[error("duplicate entity identifier type registration: {0}")]
    DuplicateType(&'static str),
}

/// Entity identifier factory backed by link-time inventory registrations.
///
/// `JandexEntityIdFactory` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct JandexEntityIdFactory {
    registrations: BTreeMap<&'static str, &'static EntityIdRegistration>,
}

impl JandexEntityIdFactory {
    /// Collects inventory registrations and rejects duplicate entity types.
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Result<Self, EntityIdRegistryError> {
        Self::from_registrations(inventory::iter::<EntityIdRegistration>)
    }

    /// Builds a factory from an explicit registration list (also used by tests).
    ///
    /// 从给定注册项构建工厂，并在遇到重复实体类型时返回 `DuplicateType`。
    pub fn from_registrations(
        registrations: impl IntoIterator<Item = &'static EntityIdRegistration>,
    ) -> Result<Self, EntityIdRegistryError> {
        let mut map = BTreeMap::new();
        for registration in registrations {
            if map
                .insert(registration.entity_type, registration)
                .is_some()
            {
                return Err(EntityIdRegistryError::DuplicateType(
                    registration.entity_type,
                ));
            }
        }
        Ok(Self {
            registrations: map,
        })
    }

    /// Returns all known entity type names in deterministic order.
    #[must_use]
    /// 执行 `id_classes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn id_classes(&self) -> Vec<&'static str> {
        self.registrations.keys().copied().collect()
    }
}

impl EntityIdFactory for JandexEntityIdFactory {
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn contains_type(&self, entity_type: &str) -> bool {
        self.registrations.contains_key(entity_type)
    }

    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn is_valid(&self, entity_type: &str, id: &str) -> bool {
        self.registrations
            .get(entity_type)
            .is_some_and(|entry| (entry.validate)(id))
    }

    /// 执行 `create_entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create_entity_id(&self, entity_type: &str, id: &str) -> Option<Box<dyn EntityId>> {
        self.registrations
            .get(entity_type)
            .and_then(|entry| (entry.parse)(id))
    }
}
