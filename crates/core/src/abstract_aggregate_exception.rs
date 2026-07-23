//! `abstract_aggregate_exception` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.AbstractAggregateException`.

/// Shared immutable data of aggregate-related failures.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `AbstractAggregateException` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractAggregateException {
    message: String,
    entity_type: String,
    entity_id: String,
}

impl AbstractAggregateException {
    /// Creates aggregate failure data.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        message: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            entity_type: entity_type.into(),
            entity_id: entity_id.into(),
        }
    }

    /// Human-readable failure message.
    #[must_use]
    /// 执行 `message` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Stable aggregate type name.
    #[must_use]
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }
    /// Aggregate identifier.
    #[must_use]
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }
}

impl std::fmt::Display for AbstractAggregateException {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AbstractAggregateException {}
