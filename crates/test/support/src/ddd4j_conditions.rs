//! `ddd4j_conditions` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.junit.Ddd4JConditions`.

use ddd_4_rust_core::{DomainEvent, EntityId, HasEntityTypeConstant};
use thiserror::Error;

/// Compile-time and naming checks replacing the Java `ArchUnit` rules.
///
/// `Ddd4JConditions` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct Ddd4JConditions;

/// A DDD architecture condition failed.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{type_name} does not satisfy {rule}")]
/// `ConditionError` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct ConditionError {
    type_name: &'static str,
    rule: &'static str,
}

impl Ddd4JConditions {
    /// Verifies the stable `Event` suffix while the generic bound proves the domain-event contract.
    ///
    /// # Errors
    /// Returns [`ConditionError`] when the concrete event type lacks the stable suffix.
    ///
    /// 执行 `domain_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn domain_event<T, Id>() -> Result<(), ConditionError>
    where
        T: DomainEvent<Id>,
        Id: EntityId + ?Sized,
    {
        let type_name = std::any::type_name::<T>();
        if type_name.ends_with("Event") {
            Ok(())
        } else {
            Err(ConditionError {
                type_name,
                rule: "domain event naming and trait contract",
            })
        }
    }

    /// Uses trait bounds to prove the entity-ID and entity-type-constant contracts.
    ///
    /// # Errors
    /// Returns [`ConditionError`] when the entity type constant is empty.
    ///
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_id<T>() -> Result<(), ConditionError>
    where
        T: EntityId + HasEntityTypeConstant,
    {
        let type_name = std::any::type_name::<T>();
        if T::ENTITY_TYPE.is_empty() {
            Err(ConditionError {
                type_name,
                rule: "non-empty ENTITY_TYPE constant",
            })
        } else {
            Ok(())
        }
    }
}
