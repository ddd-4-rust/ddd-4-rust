//! `base_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use async_trait::async_trait;
use chrono::Utc;
use ddd_4_rust_core::{
    AggregateError, AggregateRoot, AggregateRootUuid, DomainEvent, Entity, EntityId, EntityType,
};
use ddd_4_rust_esc::{CommonEvent, EventStore, EventStoreError, StreamEventsSlice, StreamId};
use std::{
    future::Future,
    task::{Context, Poll, Waker},
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
/// `StoreMode` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum StoreMode {
    ReadOne,
    ReadMissing,
    ReadEmpty,
    ReadDeleted,
    ReadConnection,
    ReadSerialization,
    ReadOther,
    AppendOk,
    AppendConflict,
    AppendMissing,
    AppendDeleted,
    AppendConnection,
    DeleteOk,
    DeleteConflict,
    DeleteConnection,
}

/// `MemoryEventStore` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct MemoryEventStore {
    /// 保存 `mode` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub mode: StoreMode,
}

/// 执行 `event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn event() -> CommonEvent {
    CommonEvent {
        event_id: Uuid::nil(),
        event_type: "VendorCreated".to_owned(),
        data: Vec::new(),
        metadata: None,
        created: Utc::now(),
        event_number: 0,
    }
}

#[async_trait]
impl EventStore for MemoryEventStore {
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_stream_events_forward(
        &self,
        stream_id: &StreamId,
        _start: i64,
        _count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        match self.mode {
            StoreMode::ReadOne => Ok(StreamEventsSlice {
                events: vec![event()],
                next_event_number: None,
                is_end_of_stream: true,
                last_event_number: Some(0),
            }),
            StoreMode::ReadMissing => Err(EventStoreError::StreamNotFound(stream_id.to_string())),
            StoreMode::ReadDeleted => Err(EventStoreError::StreamDeleted(stream_id.to_string())),
            StoreMode::ReadConnection => Err(EventStoreError::Connection("offline".into())),
            StoreMode::ReadSerialization => {
                Err(EventStoreError::Serialization("invalid event".into()))
            }
            StoreMode::ReadOther => Err(EventStoreError::Other("backend".into())),
            _ => Ok(StreamEventsSlice {
                events: Vec::new(),
                next_event_number: None,
                is_end_of_stream: true,
                last_event_number: None,
            }),
        }
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn append_to_stream(
        &self,
        _stream_id: &StreamId,
        expected_version: i64,
        _events: Vec<CommonEvent>,
    ) -> Result<(), EventStoreError> {
        match self.mode {
            StoreMode::AppendConflict => Err(EventStoreError::WrongExpectedVersion {
                expected: expected_version,
                actual: 2,
            }),
            StoreMode::AppendMissing => Err(EventStoreError::StreamNotFound("missing".into())),
            StoreMode::AppendDeleted => Err(EventStoreError::StreamDeleted("deleted".into())),
            StoreMode::AppendConnection => Err(EventStoreError::Connection("offline".into())),
            _ => Ok(()),
        }
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn delete_stream(
        &self,
        _stream_id: &StreamId,
        _expected_version: i64,
    ) -> Result<(), EventStoreError> {
        match self.mode {
            StoreMode::DeleteConflict => Err(EventStoreError::WrongExpectedVersion {
                expected: _expected_version,
                actual: 9,
            }),
            StoreMode::DeleteConnection => Err(EventStoreError::Connection("offline".into())),
            _ => Ok(()),
        }
    }
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_all_events_forward(
        &self,
        _position: i64,
        _count: u32,
    ) -> Result<StreamEventsSlice, EventStoreError> {
        Ok(StreamEventsSlice {
            events: vec![event()],
            next_event_number: None,
            is_end_of_stream: true,
            last_event_number: Some(0),
        })
    }
}

/// `VendorAggregate` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorAggregate {
    /// 保存 `id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub id: AggregateRootUuid,
    /// 保存 `version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub version: i32,
    /// 保存 `changes` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub changes: Vec<Box<dyn DomainEvent<dyn EntityId>>>,
}

impl VendorAggregate {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Result<Self, AggregateError> {
        let id = AggregateRootUuid::new("Vendor")
            .map_err(|error| AggregateError::Other(Box::new(error)))?;
        Ok(Self {
            id,
            version: -1,
            changes: Vec::new(),
        })
    }
}

impl Entity<AggregateRootUuid> for VendorAggregate {
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType {
        self.id.entity_type()
    }
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn id(&self) -> &AggregateRootUuid {
        &self.id
    }
}

impl AggregateRoot<AggregateRootUuid> for VendorAggregate {
    /// 执行 `uncommitted_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>] {
        &self.changes
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn mark_changes_as_committed(&mut self) {
        self.version = self.next_version();
        self.changes.clear();
    }
    /// 执行 `version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn version(&self) -> i32 {
        self.version
    }
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn load_from_history(
        &mut self,
        _history: &[Box<dyn DomainEvent<dyn EntityId>>],
    ) -> Result<(), AggregateError> {
        Ok(())
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn apply(&mut self, event: Box<dyn DomainEvent<dyn EntityId>>) -> Result<(), AggregateError> {
        self.changes.push(event);
        Ok(())
    }
}

/// 执行 `boxed_store` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn boxed_store(mode: StoreMode) -> Box<dyn EventStore> {
    Box::new(MemoryEventStore { mode })
}

/// 执行 `block_on` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn block_on<F: Future>(future: F) -> F::Output {
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

/// 测试用领域事件，用于覆盖 `CommonEvent` 构建路径。
pub struct StubDomainEvent {
    id: ddd_4_rust_core::EventId,
    kind: ddd_4_rust_core::EventType,
    timestamp: chrono::DateTime<chrono::Utc>,
    path: ddd_4_rust_core::EntityIdPath,
    entity: AggregateRootUuid,
    version: ddd_4_rust_core::AggregateVersion,
}

impl StubDomainEvent {
    /// 构造带指定类型与版本号的测试事件。
    pub fn new(event_type: &str, version: u32) -> Self {
        let id = AggregateRootUuid::new("Vendor").expect("aggregate id");
        let entity_id = std::sync::Arc::new(id.clone()) as std::sync::Arc<dyn EntityId>;
        Self {
            id: ddd_4_rust_core::EventId::new(),
            kind: ddd_4_rust_core::EventType::new(event_type).expect("event type"),
            timestamp: chrono::Utc::now(),
            path: ddd_4_rust_core::EntityIdPath::new(vec![entity_id]).expect("path"),
            entity: id,
            version: ddd_4_rust_core::AggregateVersion::new(version),
        }
    }
}

impl ddd_4_rust_core::Event for StubDomainEvent {
    /// 返回事件 ID。
    fn event_id(&self) -> &ddd_4_rust_core::EventId {
        &self.id
    }

    /// 返回事件类型。
    fn event_type(&self) -> &ddd_4_rust_core::EventType {
        &self.kind
    }

    /// 返回事件时间戳。
    fn event_timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }

    /// 返回关联 ID。
    fn correlation_id(&self) -> Option<&ddd_4_rust_core::EventId> {
        None
    }

    /// 返回因果 ID。
    fn causation_id(&self) -> Option<&ddd_4_rust_core::EventId> {
        None
    }
}

impl DomainEvent<dyn EntityId> for StubDomainEvent {
    /// 返回实体 ID 路径。
    fn entity_id_path(&self) -> &ddd_4_rust_core::EntityIdPath {
        &self.path
    }

    /// 返回实体 ID。
    fn entity_id(&self) -> &(dyn EntityId + 'static) {
        &self.entity
    }

    /// 返回聚合版本。
    fn aggregate_version(&self) -> Option<&ddd_4_rust_core::AggregateVersion> {
        Some(&self.version)
    }
}
