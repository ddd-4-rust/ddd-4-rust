//! `memory_event_store` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Executable contract for the shared in-memory event-store test adapter.

use chrono::Utc;
use ddd_4_rust_esc::{CommonEvent, EventStore, EventStoreError, StreamId};
use ddd_4_rust_test::{CoverageMarker, MemoryEventStore};
use std::future::Future;
use std::task::{Context, Poll, Waker};
use uuid::Uuid;

/// 执行 `block_on` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::hint::spin_loop();
    }
}

/// 执行 `event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn event(number: i64) -> CommonEvent {
    CommonEvent {
        event_id: Uuid::nil(),
        event_type: "VendorCreatedEvent".to_owned(),
        data: vec![1],
        metadata: Some(vec![2]),
        created: Utc::now(),
        event_number: number,
    }
}

#[test]
/// 执行 `memory_store_covers_append_read_delete_and_conflict` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn memory_store_covers_append_read_delete_and_conflict() -> Result<(), Box<dyn std::error::Error>> {
    let store = MemoryEventStore::default();
    let stream = StreamId::new("Vendor-1");
    block_on(store.append_to_stream(&stream, -1, vec![event(0), event(1)]))?;
    let first = block_on(store.read_stream_events_forward(&stream, 0, 1))?;
    assert_eq!(first.events.len(), 1);
    assert_eq!(first.next_event_number, Some(1));
    assert!(!first.is_end_of_stream);
    let all = block_on(store.read_all_events_forward(0, 10))?;
    assert_eq!(all.events.len(), 2);
    assert!(all.is_end_of_stream);
    assert!(matches!(
        block_on(store.append_to_stream(&stream, -1, vec![])),
        Err(EventStoreError::WrongExpectedVersion { .. })
    ));
    assert!(matches!(
        block_on(store.delete_stream(&stream, 9)),
        Err(EventStoreError::WrongExpectedVersion { .. })
    ));
    block_on(store.delete_stream(&stream, 1))?;
    assert!(
        block_on(store.read_stream_events_forward(&stream, 0, 10))?
            .events
            .is_empty()
    );
    assert_eq!(CoverageMarker::PROFILE_ENV, "LLVM_PROFILE_FILE");
    let _coverage_state = CoverageMarker::is_enabled();
    Ok(())
}
