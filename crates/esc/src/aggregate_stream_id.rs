//! `aggregate_stream_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Unique name of an aggregate stream.
//!
//! 1:1 translation of `org.fuin.ddd4j.esc.AggregateStreamId`.

use ddd_4_rust_core::aggregate_root_id::AggregateRootId;
use ddd_4_rust_core::entity_type::EntityType;

/// Unique name of an aggregate stream.
///
/// Java: `AggregateStreamId implements StreamId`
#[derive(Debug, Clone)]
/// `AggregateStreamId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateStreamId {
    entity_type: String,
    param_name: String,
    param_value: String,
}

impl AggregateStreamId {
    /// Creates a new aggregate stream ID.
    ///
    /// Java: `new AggregateStreamId(EntityType type, String paramName, AggregateRootId paramValue)`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        entity_type: &dyn EntityType,
        param_name: &str,
        param_value: &dyn AggregateRootId,
    ) -> Self {
        Self {
            entity_type: entity_type.as_string().to_string(),
            param_name: param_name.to_string(),
            param_value: param_value.as_string(),
        }
    }

    /// Returns the stream name.
    ///
    /// Java: `getName() -> String`
    #[must_use]
    /// 执行 `name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn name(&self) -> &str {
        &self.entity_type
    }

    /// Returns whether this is a projection stream.
    ///
    /// Java: `isProjection() -> boolean`
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_projection(&self) -> bool {
        false
    }

    /// Returns the string representation: `{type}-{id}`.
    ///
    /// Java: `asString() -> String`
    #[must_use]
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_string(&self) -> String {
        format!("{}-{}", self.entity_type, self.param_value)
    }

    /// Returns the parameter value as string.
    ///
    /// Java: `getSingleParamValue() -> <T> T`
    #[must_use]
    /// 执行 `param_value` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn param_value(&self) -> &str {
        &self.param_value
    }

    /// Returns the aggregate identifier parameter name.
    ///
    /// Java: `getParamNames()` / `getSingleParamName()`
    #[must_use]
    /// 执行 `param_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn param_name(&self) -> &str {
        &self.param_name
    }
}

impl std::fmt::Display for AggregateStreamId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl PartialEq for AggregateStreamId {
    /// 执行 `eq` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
    }
}

impl Eq for AggregateStreamId {}

impl std::hash::Hash for AggregateStreamId {
    /// 执行 `hash` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_string().hash(state);
    }
}
