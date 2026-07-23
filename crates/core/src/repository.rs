//! `repository` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Repository that supports CRUD operations for an aggregate.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.Repository`.

use crate::aggregate_root::AggregateRoot;
use crate::aggregate_root_id::AggregateRootId;
use crate::entity_type::EntityType;
use crate::exceptions::AggregateError;
use async_trait::async_trait;

/// Repository that supports CRUD operations for an aggregate.
///
/// Java: `Repository<ID extends AggregateRootId, T extends AggregateRoot<ID>>`
///
/// Methods (1:1):
/// - `getAggregateClass()` → `aggregate_type()` (returns EntityType instead of Class)
/// - `getAggregateType()` → `aggregate_type_name()`
/// - `create()` → `create()`
/// - `read(ID id)` → `read(id)`
/// - `read(ID id, int version)` → `read_at_version(id, version)`
/// - `update(T aggregate)` → `update(aggregate)`
/// - `update(T aggregate, String metaType, Object metaData)` → `update_with_meta(aggregate, meta_type, meta_data)`
/// - `add(T aggregate)` → `add(aggregate)`
/// - `add(T aggregate, String metaType, Object metaData)` → `add_with_meta(aggregate, meta_type, meta_data)`
/// - `delete(ID aggregateId, int expectedVersion)` → `delete(id, expected_version)`
#[async_trait]
/// `Repository` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait Repository<ID: AggregateRootId + ?Sized, T: AggregateRoot<ID> + Send>:
    Send + Sync
{
    /// Returns a unique name for the aggregate root type.
    ///
    /// Java: `getAggregateType() -> EntityType`
    ///
    /// 执行 `aggregate_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn aggregate_type(&self) -> &dyn EntityType;

    /// Factory method to create a new aggregate. Just creates a new instance without persisting.
    ///
    /// Java: `create() -> T`
    ///
    /// 执行 `create` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create(&self) -> T;

    /// Reads the latest version of an aggregate.
    ///
    /// Java: `read(ID id) -> T`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read(&self, id: &ID) -> Result<T, AggregateError>;

    /// Reads a given version of an aggregate.
    ///
    /// Java: `read(ID id, int version) -> T`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn read_at_version(&self, id: &ID, version: u32) -> Result<T, AggregateError>;

    /// Saves the changes on an aggregate in the repository.
    ///
    /// Java: `update(T aggregate)`
    ///
    /// 执行 `update` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn update(&self, aggregate: &T) -> Result<(), AggregateError>;

    /// Saves the changes on an aggregate with metadata.
    ///
    /// Java: `update(T aggregate, String metaType, Object metaData)`
    ///
    /// 执行 `update_with_meta` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn update_with_meta(
        &self,
        aggregate: &T,
        meta_type: Option<&str>,
        meta_data: Option<&[u8]>,
    ) -> Result<(), AggregateError>;

    /// Adds a new aggregate to the repository.
    ///
    /// Java: `add(T aggregate)`
    ///
    /// 执行 `add` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn add(&self, aggregate: &T) -> Result<(), AggregateError>;

    /// Adds a new aggregate to the repository with metadata.
    ///
    /// Java: `add(T aggregate, String metaType, Object metaData)`
    ///
    /// 执行 `add_with_meta` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn add_with_meta(
        &self,
        aggregate: &T,
        meta_type: Option<&str>,
        meta_data: Option<&[u8]>,
    ) -> Result<(), AggregateError>;

    /// Deletes an aggregate from the repository.
    ///
    /// Java: `delete(ID aggregateId, int expectedVersion)`
    ///
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    async fn delete(&self, id: &ID, expected_version: u32) -> Result<(), AggregateError>;
}
