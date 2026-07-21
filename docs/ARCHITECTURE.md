# ddd-4-rust 架构总览

> 本文档描述 ddd-4-rust 的整体架构、核心组件、以及与 Java ddd-4-java 的对应关系。  
> 实施细节请参阅 [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)。

---

## 一、分层架构

```text
┌─────────────────────────────────────────────────────────────┐
│  应用层（用户代码）                                            │
│    use ddd_4_rust_core::prelude::*;                         │
│    impl AggregateRoot<PersonId> for Person { ... }          │
└─────────────────────────┬───────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
┌───────▼───────┐  ┌──────▼──────┐  ┌──────▼──────┐
│  ddd-4-rust-  │  │  ddd-4-rust- │  │  ddd-4-rust- │
│  core         │  │  serde       │  │  esc          │
│  (同步)       │  │  (同步)      │  │  (async)      │
│               │  │              │  │               │
│  Event        │  │  AbstractEvent│  │  EventStore   │
│  DomainEvent  │  │  Serializer  │  │  Repository   │
│  AggregateRoot│  │  Deserializer│  │  StreamId     │
│  Repository   │  │              │  │               │
│  EntityId     │  └──────────────┘  └───────────────┘
└───────────────┘
```

## 二、核心组件映射

| Java | Rust | crate |
|---|---|---|
| `Event` | `Event` trait | core |
| `DomainEvent<ID>` | `DomainEvent<ID>` trait | core |
| `EntityId` | `EntityId` trait | core |
| `AggregateRootId` | `AggregateRootId` trait | core |
| `AggregateRoot<ID>` | `AggregateRoot<ID>` trait | core |
| `Repository<ID, T>` | `Repository<ID, T>` trait | core |
| `AbstractAggregateRoot<ID>` | `AbstractAggregateRoot<ID>` struct | core |
| `ApplyEventHandler` | `ApplyEventHandler` trait | core |
| `@ApplyEvent` | `#[apply_event]` proc-macro | codegen-processor |
| `AbstractEvent` (Jackson) | `AbstractEvent` struct | serde |
| `AbstractDomainEvent<ID>` (Jackson) | `AbstractDomainEvent` struct | serde |
| `EventStoreRepository` | `EventStoreRepository` struct | esc |

## 三、注解 → proc-macro 映射

| Java | Rust | 编译期生成 |
|---|---|---|
| `@ApplyEvent` | `#[apply_event]` | `try_apply_event(&mut self, event) -> bool` 分发 match |
| `@ChildEntityLocator` | `#[child_locator]` | 子实体查找注册 |
| `@HasEntityTypeConstant` | `#[derive(EntityId)]` | `EntityId` trait impl + `TYPE` 常量 |
| `ValueObjectProcessor` | `#[derive(DddEvent)]` | Event + DomainEvent + serde + Builder |

## 四、依赖方向

```text
ddd-4-rust-core ──→ (无内部依赖)

ddd-4-rust-serde ──→ ddd-4-rust-core
ddd-4-rust-esc ──→ ddd-4-rust-core + ddd-4-rust-serde

cqrs-4-rust-core ──→ ddd-4-rust-core
cqrs-4-rust-serde ──→ ddd-4-rust-core + ddd-4-rust-serde + cqrs-4-rust-core
cqrs-4-rust-esc ──→ cqrs-4-rust-core + ddd-4-rust-esc
cqrs-4-rust-actix ──→ cqrs-4-rust-core + cqrs-4-rust-esc
cqrs-4-rust-axum ──→ cqrs-4-rust-core + cqrs-4-rust-esc
```
