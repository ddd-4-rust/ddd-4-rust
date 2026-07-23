//! `event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Something that happened in the system.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Event`.

use crate::event_id::EventId;
use crate::event_type::EventType;
use chrono::{DateTime, Utc};
use std::any::Any;

/// Something that happened in the system.
///
/// Java: `Event extends Serializable`
///
/// Methods:
/// - `getEventId()` → `event_id()`
/// - `getEventType()` → `event_type()`
/// - `getEventTimestamp()` → `event_timestamp()`
/// - `getCorrelationId()` → `correlation_id()`
/// - `getCausationId()` → `causation_id()`
///
/// `Event` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait Event: Any + Send + Sync {
    /// Returns the identifier of the event.
    ///
    /// Java: `@NotNull getEventId()`
    ///
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_id(&self) -> &EventId;

    /// Returns the type of the event (What happened).
    ///
    /// Java: `@NotNull getEventType()`
    ///
    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_type(&self) -> &EventType;

    /// Date, time and time zone the event was created.
    ///
    /// Java: `@NotNull getEventTimestamp()`
    ///
    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_timestamp(&self) -> &DateTime<Utc>;

    /// Correlation identifier.
    ///
    /// Java: `@Nullable getCorrelationId()`
    ///
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId>;

    /// Causation identifier.
    ///
    /// Java: `@Nullable getCausationId()`
    ///
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId>;
}

/// Helper to downcast a dyn Event to a concrete type.
#[allow(dead_code)]
/// 执行 `downcast_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn downcast_event<T: Event + 'static>(event: &dyn Event) -> Option<&T> {
    (event as &dyn Any).downcast_ref::<T>()
}
