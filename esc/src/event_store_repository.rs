//! Event store repository for event-sourced aggregates.
//!
//! 1:1 translation of `org.fuin.ddd4j.esc.EventStoreRepository`.

use ddd_4_rust_core::{AggregateError, AggregateRoot, AggregateRootId};
use crate::event_store::{CommonEvent, EventStore, EventStoreError, StreamId};

/// Event store repository for event-sourced aggregates.
///
/// Provides CRUD operations backed by an event store.
///
/// Java: `EventStoreRepository<ID extends AggregateRootId, AGGREGATE extends AggregateRoot<ID>>`
///
/// # Type Parameters
/// - `ID`: The aggregate root identifier type.
/// - `A`: The aggregate type (must implement `AggregateRoot<ID>`).
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
    pub fn new(
        event_store: Box<dyn EventStore>,
        aggregate_type_name: impl Into<String>,
    ) -> Self {
        Self {
            event_store,
            aggregate_type_name: aggregate_type_name.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns a reference to the underlying event store.
    pub fn event_store(&self) -> &dyn EventStore {
        self.event_store.as_ref()
    }

    /// Builds a stream ID from the aggregate root ID.
    pub fn stream_id_for(&self, id: &ID) -> StreamId {
        StreamId::new(format!("{}-{}", self.aggregate_type_name, id.as_string()))
    }

    /// Reads an aggregate from the event store at the latest version.
    /// Requires a factory function to create and load the aggregate from events.
    pub async fn read<F>(
        &self,
        id: &ID,
        factory: F,
    ) -> Result<A, AggregateError>
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
                    return Err(AggregateError::AggregateNotFound {
                        entity_type: self.aggregate_type_name.clone(),
                        entity_id: id.as_string(),
                    });
                }
                factory(&slice.events)
            }
            Err(EventStoreError::StreamNotFound(_)) => {
                Err(AggregateError::AggregateNotFound {
                    entity_type: self.aggregate_type_name.clone(),
                    entity_id: id.as_string(),
                })
            }
            Err(EventStoreError::StreamDeleted(_)) => {
                Err(AggregateError::AggregateDeleted {
                    entity_type: self.aggregate_type_name.clone(),
                    entity_id: id.as_string(),
                })
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Reads an aggregate at a specific version.
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
            .read_stream_events_forward(&stream_id, 0, version + 1)
            .await
        {
            Ok(slice) => {
                if slice.events.is_empty() {
                    return Err(AggregateError::AggregateVersionNotFound { version });
                }
                factory(&slice.events)
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Adds a new aggregate to the event store.
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
            Ok(_) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { .. }) => {
                Err(AggregateError::AggregateAlreadyExists {
                    entity_type: self.aggregate_type_name.clone(),
                    entity_id: id.as_string(),
                })
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Updates an existing aggregate in the event store.
    pub async fn update(&self, aggregate: &A) -> Result<(), AggregateError>
    where
        A: AggregateRoot<ID>,
    {
        let id = aggregate.id();
        let stream_id = self.stream_id_for(id);
        let expected_version = (aggregate.version() - 1) as i64;
        let events = build_common_events(aggregate);

        match self
            .event_store
            .append_to_stream(&stream_id, expected_version, events)
            .await
        {
            Ok(_) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { expected, actual }) => {
                Err(AggregateError::AggregateVersionConflict {
                    expected_version: expected as u32,
                    actual_version: actual as u32,
                })
            }
            Err(EventStoreError::StreamNotFound(_)) => {
                Err(AggregateError::AggregateNotFound {
                    entity_type: self.aggregate_type_name.clone(),
                    entity_id: id.as_string(),
                })
            }
            Err(EventStoreError::StreamDeleted(_)) => {
                Err(AggregateError::AggregateDeleted {
                    entity_type: self.aggregate_type_name.clone(),
                    entity_id: id.as_string(),
                })
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }

    /// Deletes an aggregate from the event store.
    pub async fn delete(
        &self,
        id: &ID,
        expected_version: u32,
    ) -> Result<(), AggregateError> {
        let stream_id = self.stream_id_for(id);
        match self
            .event_store
            .delete_stream(&stream_id, expected_version as i64)
            .await
        {
            Ok(_) => Ok(()),
            Err(EventStoreError::WrongExpectedVersion { expected, actual }) => {
                Err(AggregateError::AggregateVersionConflict {
                    expected_version: expected as u32,
                    actual_version: actual as u32,
                })
            }
            Err(e) => Err(map_event_store_error(e)),
        }
    }
}

/// Builds CommonEvent entries from an aggregate's uncommitted changes.
fn build_common_events<ID: AggregateRootId, A: AggregateRoot<ID>>(
    aggregate: &A,
) -> Vec<CommonEvent> {
    aggregate
        .uncommitted_changes()
        .iter()
        .enumerate()
        .map(|(i, event)| CommonEvent {
            event_id: event.event_id().as_uuid().to_owned().into(),
            event_type: event.event_type().to_string(),
            data: vec![],
            metadata: None,
            created: *event.event_timestamp(),
            event_number: (aggregate.version() + i as i32) as i64,
        })
        .collect()
}

/// Maps event store errors to aggregate errors.
fn map_event_store_error(e: EventStoreError) -> AggregateError {
    AggregateError::Other(Box::new(e))
}
