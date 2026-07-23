//! `event_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Universal unique event identifier.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EventId`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Universal unique event identifier.
///
/// Java: `EventId extends AbstractUuidValueObject implements TechnicalId, AsStringCapable, Serializable`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// `EventId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EventId(Uuid);

impl EventId {
    /// Creates a new random event identifier.
    ///
    /// Java: `new EventId()`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an event identifier from a UUID.
    ///
    /// Java: `new EventId(UUID uuid)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying UUID.
    ///
    /// Java: `asBaseType()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Returns the string representation.
    ///
    /// Java: `asString()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Converts a string into an event identifier.
    ///
    /// Java: `EventId.valueOf(String value)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of(value: &str) -> Option<Self> {
        Uuid::parse_str(value).ok().map(Self)
    }
}

impl Default for EventId {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for EventId {
    /// 执行 `from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<EventId> for Uuid {
    /// 执行 `from` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn from(event_id: EventId) -> Self {
        event_id.0
    }
}
