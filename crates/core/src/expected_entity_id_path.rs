//! `expected_entity_id_path` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.ExpectedEntityIdPath`.

/// Ordered entity-type contract for an [`crate::EntityIdPath`].
#[derive(Debug, Clone, PartialEq, Eq)]
/// `ExpectedEntityIdPath` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct ExpectedEntityIdPath {
    entity_types: Vec<String>,
}

impl ExpectedEntityIdPath {
    /// Creates an ordered expected path.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(entity_types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            entity_types: entity_types.into_iter().map(Into::into).collect(),
        }
    }
    /// Expected entity type names.
    #[must_use]
    /// 执行 `entity_types` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_types(&self) -> &[String] {
        &self.entity_types
    }
}
