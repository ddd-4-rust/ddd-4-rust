//! Cache for aggregates of the same type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.AggregateCache` and `AggregateNoCache`.

use crate::aggregate_root_id::AggregateRootId;

/// Cache for aggregates of the same type.
///
/// Java: `AggregateCache<AGGREGATE>`
pub trait AggregateCache<A>: Send + Sync {
    /// Tries to read the aggregate with the given identifier from the cache.
    ///
    /// Java: `get(AggregateRootId aggregateId, Integer version) -> AGGREGATE`
    fn get(&self, aggregate_id: &dyn AggregateRootId, version: Option<u32>) -> Option<A>;

    /// Puts an aggregate with the given identifier in the cache.
    ///
    /// Java: `put(AggregateRootId aggregateId, AGGREGATE aggregate)`
    fn put(&self, aggregate_id: &dyn AggregateRootId, aggregate: A);

    /// Removes the aggregate with the given identifier from the cache.
    ///
    /// Java: `remove(AggregateRootId aggregateId)`
    fn remove(&self, aggregate_id: &dyn AggregateRootId);
}

/// Never caches anything.
///
/// Java: `AggregateNoCache<AGGREGATE> implements AggregateCache<AGGREGATE>`
pub struct AggregateNoCache<A> {
    _phantom: std::marker::PhantomData<A>,
}

impl<A> AggregateNoCache<A> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<A: Send + Sync> Default for AggregateNoCache<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Send + Sync> AggregateCache<A> for AggregateNoCache<A> {
    fn get(&self, _aggregate_id: &dyn AggregateRootId, _version: Option<u32>) -> Option<A> {
        None
    }

    fn put(&self, _aggregate_id: &dyn AggregateRootId, _aggregate: A) {
        // Do nothing
    }

    fn remove(&self, _aggregate_id: &dyn AggregateRootId) {
        // Do nothing
    }
}
