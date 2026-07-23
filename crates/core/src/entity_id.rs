//! `entity_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Identifies an entity within all entities of the same type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityId`.

use crate::entity_id_factory::EntityIdFactory;
use crate::entity_type::EntityType;
use std::fmt::{Debug, Display};

/// Identifies an entity within all entities of the same type.
///
/// Java: `EntityId extends TechnicalId, AsStringCapable, Serializable`
///
/// Each implementor must also implement `Display` and `Debug`.
///
/// `EntityId` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EntityId: Debug + Display + Send + Sync + 'static {
    /// Returns the type represented by this identifier.
    ///
    /// Java: `getType() -> EntityType`
    ///
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType;

    /// Returns the entity identifier as string.
    ///
    /// Java: `asString()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> String;

    /// Returns the entity identifier as string with type and identifier.
    ///
    /// Java: `asTypedString()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type().as_string(), self.as_string())
    }

    /// Verifies that the given value can be converted into an entity identifier.
    ///
    /// Java: `EntityId.isValid(EntityIdFactory factory, String value)`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn is_valid(factory: &dyn EntityIdFactory, value: Option<&str>) -> bool
    where
        Self: Sized,
    {
        let Some(value) = value else { return true };
        let Some(p) = value.find(' ') else {
            return false;
        };
        let r#type = &value[..p];
        let id = &value[p + 1..];
        if !factory.contains_type(r#type) {
            return false;
        }
        factory.is_valid(r#type, id)
    }

    /// Converts a string into an entity identifier using the given factory.
    ///
    /// Java: `EntityId.valueOf(EntityIdFactory factory, String value)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn value_of(factory: &dyn EntityIdFactory, value: Option<&str>) -> Option<Box<dyn EntityId>>
    where
        Self: Sized,
    {
        let value = value?;
        let p = value.find(' ')?;
        let r#type = &value[..p];
        let id = &value[p + 1..];
        factory.create_entity_id(r#type, id)
    }
}
