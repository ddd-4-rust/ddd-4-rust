# Java/Rust 核心契约设计

**日期**: 2026-07-23
**作用范围**: `ddd-4-rust-core`。
**类型**: 长期架构事实。

---

## 1. 契约对照表

| Java | Rust | 所在 crate |
|---|---|---|
| `Event` | `Event` trait | core |
| `DomainEvent<ID>` | `DomainEvent<ID>` trait | core |
| `EntityId` | `EntityId` trait | core |
| `AggregateRootId` | `AggregateRootId` trait | core |
| `AggregateRoot<ID>` | `AggregateRoot<ID>` trait | core |
| `Repository<ID, T>` | 异步 `Repository<ID, T>` trait | core |
| `AbstractAggregateRoot<ID>` | `AbstractAggregateRoot<ID>` | core |
| `ApplyEventHandler` | `ApplyEventHandler` trait | core |
| Java 异常 | 独立 Rust 错误 + `AggregateError` | core |
| `EventStoreRepository` | `EventStoreRepository` | esc |

## 2. 同步/异步边界

- 领域对象（聚合根、实体、值对象、事件、异常）保持 **同步**。
- `Repository<ID, T>` 与 Event Store I/O 端口（`EventStore`,
  `EventStoreRepository`）保持 **异步**，通过 `async-trait` 暴露。
- 不为同步路径提供 `block_on` 包装；调用方自行选择运行时。

## 3. ApplyEventHandler 结果语义

事件分发使用结构化结果：

```text
ApplyEventHandler::try_apply_event
├── Ok(true)  → 事件已应用
├── Ok(false) → 没有处理器，由聚合根转换为 EventHandlerNotFound
└── Err(e)    → 处理器执行失败，保留具体错误
```

聚合根内部把 `Ok(false)` 转换为 `EventHandlerNotFound`，对调用方暴露
"未找到"与"执行失败"的区分。

## 4. 错误建模

- 每个 Java 异常对应独立的 PascalCase Rust 错误类型。
- `AggregateError` 作为稳定统一入口提供结构化包装与 `From` 转换。
- 历史回放、事件应用、构建校验、加解密、版本推进均通过 `Result`
  暴露失败，**禁止**在生产路径用 `panic!` / `unwrap` / `expect` /
  `unimplemented` 表达可恢复错误。
- 仅在以下场景允许 panic：编译期 `const_new` 失败、初始化失败的 abort。
  这些场景必须用 `audit_rust_conventions.py` 的 deny 规则拦截。

## 5. 时间值

`ZonedDateTimeValue` 同时保存：

- UTC 瞬时值（用于序列化往返）；
- `chrono_tz::Tz` 的 IANA 名称（用于恢复 `Europe/Berlin`、DST、
  非法时区等场景的原始区域信息）。

不得仅保存 `chrono::DateTime<Utc>`，否则序列化往返后区域信息丢失。

## 6. 版本推进

`AggregateRoot::increment_version` 必须在每次成功应用事件后调用。
历史回放期间禁止调用；仅在新事件被追加到流之后调用一次。

## 7. 类型常量

`@HasEntityTypeConstant` 在 Java 中生成的 ENTITY_TYPE 常量在 Rust 中
由 `#[derive(EntityId)]` 自动生成，作为关联函数 `entity_type()` 返回。
`AggregateRoot<ID>::aggregate_type()` 必须返回稳定字符串，事件存储
按该字符串分类事件流。