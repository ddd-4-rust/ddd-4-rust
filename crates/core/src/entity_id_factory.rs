//! `entity_id_factory` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Factory to create entity identifiers.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityIdFactory`.

use crate::entity_id::EntityId;

/// Factory to create entity identifiers.
///
/// Java: `EntityIdFactory`
///
/// `EntityIdFactory` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait EntityIdFactory: Send + Sync {
    /// Verifies if the given type string is a valid one.
    ///
    /// Java: `containsType(String type) -> boolean`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn contains_type(&self, r#type: &str) -> bool;

    /// Determines if an identifier of the given type is valid.
    ///
    /// Java: `isValid(String type, String id) -> boolean`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn is_valid(&self, r#type: &str, id: &str) -> bool;

    /// Creates an entity id by type and string identifier.
    ///
    /// Java: `createEntityId(String type, String id) -> EntityId`
    ///
    /// 执行 `create_entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create_entity_id(&self, r#type: &str, id: &str) -> Option<Box<dyn EntityId>>;
}
