//! `abstract_aggregate_root_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::base_test::TestEntityId;
use chrono::{DateTime, Utc};
use ddd_4_rust_core::{
    AbstractAggregateRoot, AggregateError, AggregateRootUuid, AggregateVersion, ApplyEventHandler,
    DomainEvent, EntityId, EntityIdPath, Event, EventId, EventType,
};
use std::{
    any::TypeId,
    sync::{Arc, OnceLock},
};

/// `TestEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct TestEvent {
    id: EventId,
    kind: EventType,
    timestamp: DateTime<Utc>,
    path: EntityIdPath,
    version: AggregateVersion,
}

impl TestEvent {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn new(kind: &str, version: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let id = Arc::new(TestEntityId::new("A", 1)?) as Arc<dyn EntityId>;
        Ok(Self {
            id: EventId::new(),
            kind: EventType::new(kind)?,
            timestamp: "2020-01-01T00:00:00Z".parse()?,
            path: EntityIdPath::new(vec![id])?,
            version: AggregateVersion::new(version),
        })
    }
}

impl Event for TestEvent {
    /// 执行 `event_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_id(&self) -> &EventId {
        &self.id
    }
    /// 执行 `event_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_type(&self) -> &EventType {
        &self.kind
    }
    /// 执行 `event_timestamp` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
    /// 执行 `correlation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn correlation_id(&self) -> Option<&EventId> {
        None
    }
    /// 执行 `causation_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

impl DomainEvent<dyn EntityId> for TestEvent {
    /// 执行 `entity_id_path` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id_path(&self) -> &EntityIdPath {
        &self.path
    }
    /// 执行 `entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_id(&self) -> &(dyn EntityId + 'static) {
        self.path.last().as_ref()
    }
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        Some(&self.version)
    }
}

#[derive(Default)]
/// `Handler` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
struct Handler {
    applied: Vec<String>,
    ignore_all: bool,
}

impl ApplyEventHandler for Handler {
    /// 执行 `try_apply_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn try_apply_event(
        &mut self,
        event: &dyn DomainEvent<dyn EntityId>,
    ) -> Result<bool, AggregateError> {
        match event.event_type().as_str() {
            "Unknown" => Ok(false),
            "Failure" => Err(AggregateError::Other(Box::new(std::io::Error::other(
                "handler failed",
            )))),
            kind => {
                self.applied.push(kind.to_owned());
                Ok(true)
            }
        }
    }
    /// 执行 `ignored_events` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn ignored_events(&self) -> &[TypeId] {
        static IGNORED: OnceLock<Vec<TypeId>> = OnceLock::new();
        if self.ignore_all {
            IGNORED.get_or_init(|| vec![TypeId::of::<TestEvent>()])
        } else {
            &[]
        }
    }
}

/// 执行 `root` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn root() -> AbstractAggregateRoot<AggregateRootUuid> {
    AbstractAggregateRoot::new()
}
/// 执行 `boxed_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn boxed_event(
    kind: &str,
    version: u32,
) -> Result<Box<dyn DomainEvent<dyn EntityId>>, Box<dyn std::error::Error>> {
    Ok(Box::new(TestEvent::new(kind, version)?))
}

#[test]
/// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn new_root_starts_at_version_minus_one() {
    assert_eq!(root().version(), -1);
}
#[test]
/// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn new_root_has_no_uncommitted_changes() {
    assert!(!root().has_uncommitted_changes());
}
#[test]
/// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn new_root_next_apply_version_is_zero() {
    assert_eq!(root().next_apply_version(), AggregateVersion::new(0));
}
#[test]
/// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn applying_event_invokes_handler() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    let mut handler = Handler::default();
    root.apply(&mut handler, boxed_event("Created", 0)?)?;
    assert_eq!(handler.applied, vec!["Created"]);
    Ok(())
}
#[test]
/// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn applying_event_records_uncommitted_change() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    root.apply(&mut Handler::default(), boxed_event("Created", 0)?)?;
    assert_eq!(root.uncommitted_changes().len(), 1);
    assert_eq!(root.next_version(), 0);
    Ok(())
}
#[test]
/// 执行 `missing_handler_returns_structured_error` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn missing_handler_returns_structured_error() -> Result<(), Box<dyn std::error::Error>> {
    let error = root()
        .apply(&mut Handler::default(), boxed_event("Unknown", 0)?)
        .err();
    assert!(matches!(
        error,
        Some(AggregateError::EventHandlerNotFound { .. })
    ));
    Ok(())
}
#[test]
/// 执行 `handler_failure_is_propagated` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn handler_failure_is_propagated() -> Result<(), Box<dyn std::error::Error>> {
    let error = root()
        .apply(&mut Handler::default(), boxed_event("Failure", 0)?)
        .err();
    assert_eq!(
        error.map(|e| e.to_string()),
        Some("handler failed".to_owned())
    );
    Ok(())
}
#[test]
/// 执行 `committing_clears_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn committing_clears_changes() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    root.apply(&mut Handler::default(), boxed_event("Created", 0)?)?;
    root.mark_changes_as_committed();
    assert!(!root.has_uncommitted_changes());
    Ok(())
}
#[test]
/// 执行 `committing_advances_version_by_change_count` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn committing_advances_version_by_change_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    let mut handler = Handler::default();
    root.apply(&mut handler, boxed_event("Created", 0)?)?;
    root.apply(&mut handler, boxed_event("Renamed", 1)?)?;
    root.mark_changes_as_committed();
    assert_eq!(root.version(), 1);
    Ok(())
}
#[test]
/// 执行 `history_replay_invokes_handlers` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn history_replay_invokes_handlers() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    let mut handler = Handler::default();
    root.load_from_history(
        &mut handler,
        &[boxed_event("Created", 0)?, boxed_event("Renamed", 1)?],
    )?;
    assert_eq!(handler.applied, vec!["Created", "Renamed"]);
    Ok(())
}
#[test]
/// 执行 `history_replay_does_not_create_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn history_replay_does_not_create_changes() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    root.load_from_history(&mut Handler::default(), &[boxed_event("Created", 0)?])?;
    assert!(!root.has_uncommitted_changes());
    assert_eq!(root.version(), 0);
    Ok(())
}
#[test]
/// 执行 `history_unknown_event_returns_error` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn history_unknown_event_returns_error() -> Result<(), Box<dyn std::error::Error>> {
    let error = root()
        .load_from_history(&mut Handler::default(), &[boxed_event("Unknown", 0)?])
        .err();
    assert!(matches!(
        error,
        Some(AggregateError::EventHandlerNotFound { .. })
    ));
    Ok(())
}
#[test]
/// 执行 `ignored_history_event_does_not_advance_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn ignored_history_event_does_not_advance_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = root();
    let mut handler = Handler {
        applied: Vec::new(),
        ignore_all: true,
    };
    root.load_from_history(&mut handler, &[boxed_event("Ignored", 0)?])?;
    assert_eq!(root.version(), -1);
    assert!(handler.applied.is_empty());
    Ok(())
}
