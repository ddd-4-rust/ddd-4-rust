# ESC Event Store 适配设计

**日期**: 2026-07-23
**作用范围**: `ddd-4-rust-esc`。
**类型**: 长期架构事实。

---

## 1. 设计边界

`ddd-4-rust-esc` 提供异步 Event Store 适配：

- `EventStore` trait：底层事件存储读/写/订阅接口；
- `EventStoreRepository`：仓储模式实现，按聚合根 ID 加载/追加事件流；
- `AggregateStreamId`：聚合流 ID 的强类型包装，避免字符串误用。

**强制约束**:

- `esc` 只依赖 `ddd-4-rust-core`；
- 不为未使用的线格式能力依赖 `ddd-4-rust-serde`；
- 不引入具体 Event Store 实现（如 KurrentDB / EventStoreDB 客户端），
  具体实现由使用方在 adapter crate 提供。

## 2. 错误传播

ESC 覆盖以下错误路径：

1. 读取事件流（按 ID 或全局订阅）；
2. 指定版本读取；
3. 追加事件（含乐观锁版本检查）；
4. 并发冲突（`ExpectedVersionConflict` 等价）；
5. 删除事件流（GDPR 等场景）；
6. 缓存命中（`AggregateCache` 命中直接返回 `Ok(CachedAggregate)`）；
7. 底层 IO 错误映射（网络、序列化、超时）。

每个错误路径必须返回 `Result<_, AggregateError>`（或独立错误
类型，通过 `From` 聚合），保留原始 cause 用于日志。

## 3. 缓存契约

`AggregateCache` 与 `AggregateNoCache` 必须：

- 实现同一 trait（`AggregateCachePolicy`）；
- 提供 `get(aggregate_id) -> Option<AggregateSnapshot>` 与
  `put(snapshot)`；
- `AggregateNoCache` 的 `get` 永远返回 `None`，`put` 为 no-op。

`esc` 通过 `Arc<dyn AggregateCachePolicy>` 注入；不强制存在缓存。

## 4. 并发与事务

`EventStoreRepository::append` 必须：

- 接受当前聚合版本（来自 `AggregateRoot::version()`）；
- 在 Event Store 端执行 `expected_version` 检查；
- 失败时返回 `AggregateVersionConflictException`，**不重试**。

调用方负责重试或上抛。重试必须由领域层显式控制，不能隐藏在
基础设施层。

## 5. 删除与 GDPR

`delete(aggregate_id)` 必须：

- 从 Event Store 中逻辑删除（追加 `AggregateDeletedEvent`）；
- 触发 `AggregateDeletedException` 对应的领域规则；
- 不物理删除历史事件（保留可审计性）。

GDPR "right to be forgotten" 通过 `EncryptedData` 的密钥销毁实现，
不在 ESC 层处理。

## 6. 与 ddd-4-java 的对应

| Java | Rust |
|---|---|
| `EventStore` (interface) | `EventStore` trait |
| `EventStoreRepository` | `EventStoreRepository` |
| `AggregateStreamId` | `AggregateStreamId` |
| `AggregateCache` / `AggregateNoCache` | 同名 struct（实现同一 trait） |
| `AggregateAlreadyExistsException` | `AggregateAlreadyExistsException` |
| `AggregateDeletedException` | `AggregateDeletedException` |
| `AggregateVersionConflictException` | `AggregateVersionConflictException` |
| `AggregateVersionNotFoundException` | `AggregateVersionNotFoundException` |