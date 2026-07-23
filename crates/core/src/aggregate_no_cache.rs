//! `aggregate_no_cache` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.AggregateNoCache`.

use crate::{AggregateCache, AggregateRootId};

/// Cache implementation that intentionally never stores aggregates.
///
/// `AggregateNoCache` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateNoCache<A>(std::marker::PhantomData<A>);

impl<A> AggregateNoCache<A> {
    /// Creates an empty no-cache adapter.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<A> Default for AggregateNoCache<A> {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Send + Sync> AggregateCache<A> for AggregateNoCache<A> {
    /// 执行 `get` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn get(&self, _aggregate_id: &dyn AggregateRootId, _version: Option<u32>) -> Option<A> {
        None
    }

    /// 执行 `put` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn put(&self, _aggregate_id: &dyn AggregateRootId, _aggregate: A) {}

    /// 执行 `remove` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn remove(&self, _aggregate_id: &dyn AggregateRootId) {}
}
