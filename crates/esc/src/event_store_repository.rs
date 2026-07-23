//! `event_store_repository` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Event store repository for event-sourced aggregates.
//!
//! 1:1 translation of `org.fuin.ddd4j.esc.EventStoreRepository`.

use crate::event_store::{CommonEvent, EventStore, EventStoreError, StreamId};
use ddd_4_rust_core::{
    AggregateAlreadyExistsException, AggregateDeletedException, AggregateError,
    AggregateNotFoundException, AggregateRoot, AggregateRootId, AggregateVersionConflictException,
    AggregateVersionNotFoundException,
};

/// Event store repository for event-sourced aggregates.
///
/// Provides CRUD operations backed by an event store.
///
/// Java: `EventStoreRepository<ID extends AggregateRootId, AGGREGATE extends AggregateRoot<ID>>`
///
/// # Type Parameters
/// - `ID`: The aggregate root identifier type.
/// - `A`: The aggregate type (must implement `AggregateRoot<ID>`).
///
/// `EventStoreRepository` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EventStoreRepository<ID: AggregateRootId, A> {
    event_store: Box<dyn EventStore>,
    aggregate_type_name: String,
    _phantom: std::marker::PhantomData<(ID, A)>,
}

impl<ID: AggregateRootId, A> EventStoreRepository<ID, A>
where
    A: AggregateRoot<ID> + Send + Sync,
    ID: 'static,
{
    /// Creates a new event store repository.
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(event_store: Box<dyn EventStore>, aggregate_type_name: impl Into<String>) -> Self {
        Self {
            event_store,
            aggregate_type_name: aggregate_type_name.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns a reference to the underlying event store.
    #[must_use]
    /// 执行 `event_store` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn event_store(&self) -> &dyn EventStore {
        self.event_store.as_ref()
    }

    /// Builds a stream ID from the aggregate root ID.
    ///
    /// 执行 `stream_id_for` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn stream_id_for(&self, id: &ID) -> StreamId {
        StreamId::new(format!("{}-{}", self.aggregate_type_name, id.as_string()))
    }

    /// Reads an aggregate from the event store at the latest version.
    /// Requires a factory function to create and load the aggregate from events.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub async fn read<F>(&self, id: &ID, factory: F) -> Result<A, AggregateError>
    where
        F: FnOnce(&[CommonEvent]) -> Result<A, AggregateError>,
    {
        let stream_id = self.stream_id_for(id);
        match self
            .event_store
            .read_stream_events_forward(&stream_id, 0, 4096)
            .await
        {
            Ok(slice) => {
                if slice.events.is_empty() {
                    return Err(AggregateNotFoundException::new(
                        self.aggregate_type_name.clone(),
                        id.as_string(),
                    )
                    .into());
                }
                factory(&slice.events)
            }
            Err(EventStoreError::StreamNotFound(_)) => Err(AggregateNotFoundException::new(
                self.aggregate_type_name.clone(),
                id.as_string(),
            )
            .into()),
            Err(EventStoreError::StreamDeleted(_)) => Err(AggregateDeletedException::new(
                self.aggregate_type_name.clone(),
                id.as_string(),
            )
            .into()),
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Reads an aggregate at a specific version.
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub async fn read_at_version<F>(
        &self,
        id: &ID,
        version: u32,
        factory: F,
    ) -> Result<A, AggregateError>
    where
        F: FnOnce(&[CommonEvent]) -> Result<A, AggregateError>,
    {
        let stream_id = self.stream_id_for(id);
        match self
            .event_store
            .read_stream_events_forward(&stream_id, 0, version.saturating_add(1))
            .await
        {
            Ok(slice) => {
                if slice.events.is_empty() {
                    return Err(AggregateVersionNotFoundException::new(
                        self.aggregate_type_name.clone(),
                        id.as_string(),
                        i32::try_from(version).unwrap_or(i32::MAX),
                    )
                    .into());
                }
                factory(&slice.events)
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Adds a new aggregate to the event store.
    ///
    /// 执行 `add` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub async fn add(&self, aggregate: &A) -> Result<(), AggregateError>
    where
        A: AggregateRoot<ID>,
    {
        let id = aggregate.id();
        let stream_id = self.stream_id_for(id);
        let events = build_common_events(aggregate);

        match self
            .event_store
            .append_to_stream(&stream_id, -1, events)
            .await
        {
            Ok(()) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { .. }) => {
                Err(AggregateAlreadyExistsException::new(
                    self.aggregate_type_name.clone(),
                    id.as_string(),
                    aggregate.version(),
                )
                .into())
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Updates an existing aggregate in the event store.
    ///
    /// 执行 `update` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub async fn update(&self, aggregate: &A) -> Result<(), AggregateError>
    where
        A: AggregateRoot<ID>,
    {
        let id = aggregate.id();
        let stream_id = self.stream_id_for(id);
        let expected_version = i64::from(aggregate.version() - 1);
        let events = build_common_events(aggregate);

        match self
            .event_store
            .append_to_stream(&stream_id, expected_version, events)
            .await
        {
            Ok(()) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { expected, actual }) => {
                Err(AggregateVersionConflictException::new(
                    self.aggregate_type_name.clone(),
                    id.as_string(),
                    i32::try_from(expected).unwrap_or(i32::MAX),
                    i32::try_from(actual).unwrap_or(i32::MAX),
                )
                .into())
            }
            Err(EventStoreError::StreamNotFound(_)) => Err(AggregateNotFoundException::new(
                self.aggregate_type_name.clone(),
                id.as_string(),
            )
            .into()),
            Err(EventStoreError::StreamDeleted(_)) => Err(AggregateDeletedException::new(
                self.aggregate_type_name.clone(),
                id.as_string(),
            )
            .into()),
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Deletes an aggregate from the event store.
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub async fn delete(&self, id: &ID, expected_version: u32) -> Result<(), AggregateError> {
        let stream_id = self.stream_id_for(id);
        match self
            .event_store
            .delete_stream(&stream_id, i64::from(expected_version))
            .await
        {
            Ok(()) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { expected, actual }) => {
                Err(AggregateVersionConflictException::new(
                    self.aggregate_type_name.clone(),
                    id.as_string(),
                    i32::try_from(expected).unwrap_or(i32::MAX),
                    i32::try_from(actual).unwrap_or(i32::MAX),
                )
                .into())
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }
}

/// Builds `CommonEvent` entries from an aggregate's uncommitted changes.
///
/// 根据已收集的参数构建领域对象；无效组合通过返回类型显式报告。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn build_common_events<ID: AggregateRootId, A: AggregateRoot<ID>>(
    aggregate: &A,
) -> Vec<CommonEvent> {
    aggregate
        .uncommitted_changes()
        .iter()
        .enumerate()
        .map(|(i, event)| CommonEvent {
            event_id: event.event_id().as_uuid().to_owned(),
            event_type: event.event_type().to_string(),
            data: vec![],
            metadata: None,
            created: *event.event_timestamp(),
            event_number: i64::from(aggregate.version())
                .saturating_add(i64::try_from(i).unwrap_or(i64::MAX)),
        })
        .collect()
}

/// Maps event store errors to aggregate errors.
///
/// 执行 `map_event_store_error` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn map_event_store_error(e: EventStoreError) -> AggregateError {
    AggregateError::Other(Box::new(e))
}
