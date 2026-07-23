//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! DDD-4-Rust Test: Test utilities.
//!
//! 1:1 translation of `ddd-4-java-test`.
//!
//! Provides mock event store and test helpers.

use async_trait::async_trait;
use ddd_4_rust_esc::{CommonEvent, EventStore, EventStoreError, StreamEventsSlice, StreamId};
use std::sync::{Mutex, MutexGuard};

mod coverage_marker;
mod ddd4j_conditions;

pub use coverage_marker::CoverageMarker;
pub use ddd4j_conditions::{ConditionError, Ddd4JConditions};

/// A simple in-memory event store for testing.
///
/// `MemoryEventStore` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct MemoryEventStore {
    events: Mutex<std::collections::HashMap<String, Vec<CommonEvent>>>,
}

impl MemoryEventStore {
    /// Creates an empty in-memory event store.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Self {
        Self {
            events: Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// 执行 `events` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn events(
        &self,
    ) -> Result<MutexGuard<'_, std::collections::HashMap<String, Vec<CommonEvent>>>, EventStoreError>
    {
        self.events
            .lock()
            .map_err(|_| EventStoreError::Other("memory event store lock poisoned".to_owned()))
    }
}

impl Default for MemoryEventStore {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for MemoryEventStore {
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_stream_events_forward(
        &self,
        stream_id: &StreamId,
        start: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        let events_map = self.events()?;
        let events = events_map
            .get(stream_id.as_str())
            .cloned()
            .unwrap_or_default();
        let start_idx = usize::try_from(start.max(0))
            .unwrap_or(usize::MAX)
            .min(events.len());
        let end_idx = start_idx
            .saturating_add(usize::try_from(count).unwrap_or(usize::MAX))
            .min(events.len());
        let slice_events: Vec<CommonEvent> = events[start_idx..end_idx].to_vec();
        let last_event_number = events.last().map(|e| e.event_number);
        Ok(StreamEventsSlice {
            events: slice_events,
            next_event_number: if end_idx < events.len() {
                Some(i64::try_from(end_idx).unwrap_or(i64::MAX))
            } else {
                None
            },
            is_end_of_stream: end_idx >= events.len(),
            last_event_number,
        })
    }

    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn append_to_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
        events: Vec<CommonEvent>,
    ) -> Result<(), EventStoreError> {
        let mut events_map = self.events()?;
        let stream = events_map
            .entry(stream_id.as_str().to_string())
            .or_default();
        let actual_version = if stream.is_empty() {
            -1
        } else {
            i64::try_from(stream.len()).unwrap_or(i64::MAX) - 1
        };
        if expected_version != -2 && expected_version != actual_version {
            return Err(EventStoreError::WrongExpectedVersion {
                expected: expected_version,
                actual: actual_version,
            });
        }
        stream.extend(events);
        Ok(())
    }

    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn delete_stream(
        &self,
        stream_id: &StreamId,
        expected_version: i64,
    ) -> Result<(), EventStoreError> {
        let mut events_map = self.events()?;
        let stream = events_map.get(stream_id.as_str());
        let actual_version = match stream {
            Some(s) if !s.is_empty() => i64::try_from(s.len()).unwrap_or(i64::MAX) - 1,
            _ => -1,
        };
        if expected_version != actual_version {
            return Err(EventStoreError::WrongExpectedVersion {
                expected: expected_version,
                actual: actual_version,
            });
        }
        events_map.remove(stream_id.as_str());
        Ok(())
    }

    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_all_events_forward(
        &self,
        position: i64,
        count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        let events_map = self.events()?;
        let mut events: Vec<CommonEvent> = events_map
            .values()
            .flat_map(|stream| stream.iter().cloned())
            .collect();
        events.sort_by_key(|event| event.event_number);
        let start = usize::try_from(position.max(0))
            .unwrap_or(usize::MAX)
            .min(events.len());
        let end = start
            .saturating_add(usize::try_from(count).unwrap_or(usize::MAX))
            .min(events.len());
        let selected = events.get(start..end).unwrap_or_default().to_vec();
        let last_event_number = events.last().map(|event| event.event_number);
        Ok(StreamEventsSlice {
            events: selected,
            next_event_number: (end < events.len())
                .then_some(i64::try_from(end).unwrap_or(i64::MAX)),
            is_end_of_stream: end >= events.len(),
            last_event_number,
        })
    }
}
