//! `aggregate_cache` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Cache for aggregates of the same type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateCache`.

use crate::aggregate_root_id::AggregateRootId;

/// Cache for aggregates of the same type.
///
/// Java: `AggregateCache<AGGREGATE>`
///
/// `AggregateCache` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait AggregateCache<A>: Send + Sync {
    /// Tries to read the aggregate with the given identifier from the cache.
    ///
    /// Java: `get(AggregateRootId aggregateId, Integer version) -> AGGREGATE`
    ///
    /// 执行 `get` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn get(&self, aggregate_id: &dyn AggregateRootId, version: Option<u32>) -> Option<A>;

    /// Puts an aggregate with the given identifier in the cache.
    ///
    /// Java: `put(AggregateRootId aggregateId, AGGREGATE aggregate)`
    ///
    /// 执行 `put` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn put(&self, aggregate_id: &dyn AggregateRootId, aggregate: A);

    /// Removes the aggregate with the given identifier from the cache.
    ///
    /// Java: `remove(AggregateRootId aggregateId)`
    ///
    /// 执行 `remove` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn remove(&self, aggregate_id: &dyn AggregateRootId);
}
