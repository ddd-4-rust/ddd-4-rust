# ddd-4-rust

> **Idiomatic Rust port of [`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java)** —
> Domain-Driven Design building blocks for Rust.
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2021-orange)](https://doc.rust-lang.org/edition-guide/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v2-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.7.0-blue)](https://github.com/ddd-4-rust/ddd-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/Port%20of-fuinorg/ddd--4--java-green?logo=github)](https://github.com/fuinorg/ddd-4-java)
[![Progress](https://img.shields.io/badge/Migration-58%25-yellow)](docs/MIGRATION_STATUS.md)

---

## 🙏 Acknowledgement — Java Source / 致谢

This project is a **one-to-one Rust translation** of:

> **[`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java)**
> by [Michael Schnell / fuinorg](https://github.com/fuinorg)
> *"Base classes for Domain Driven Design (DDD) with Java"*
>
> 📜 Original Java source license: **LGPLv3**
> 📌 Source version base: **0.7.0**

All credit for the **original architecture, API design, and Domain-Driven Design insights** goes to the **`fuinorg`** project and its contributors. This Rust port exists to bring the same battle-tested DDD primitives into the Rust ecosystem with **idiomatic type safety, ownership semantics, and async-first design**.

| | Java (source) | Rust (this port) |
|---|---|---|
| **Repository** | [fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java) | [ddd-4-rust/ddd-4-rust](https://github.com/ddd-4-rust/ddd-4-rust) |
| **Owner** | [fuinorg](https://github.com/fuinorg) | [ddd-4-rust org](https://github.com/ddd-4-rust) |
| **Maintainer** | Michael Schnell | hiwepy |
| **Language** | Java 17 | Rust 2021 |
| **License** | LGPLv3 | Apache 2.0 |
| **Version base** | 0.7.0 | 0.7.0 |
| **API compatibility** | — | 1:1 functional (idiomatic where required) |

---

## 🎯 What is this?

**`ddd-4-rust`** is a Cargo workspace providing **DDD building blocks** for Rust:

- **`Entity<TId>`** — strongly-typed entity root
- **`AggregateRoot<TId, TEvent>`** — aggregate with internal event sourcing
- **`DomainEvent`** — base trait for all domain events
- **`EntityId<TId>`** — newtype-style ID with serialization support
- **`Repository<T, TId>`** trait — persistence-agnostic storage SPI
- **`#[apply_event]`** — proc-macro that generates `try_apply_event` dispatch
- **`AggregateStreamId`** — stream identifier for event store

Built on Rust's type system: **newtype patterns, trait constraints, and PhantomData** make invalid domain states unrepresentable.

---

## 🧱 Workspace Architecture / Workspace 结构

```text
ddd-4-rust/                     ← Cargo Workspace (resolver = 2)
├── core/                       ← 核心：Event / DomainEvent / AggregateRoot / EntityId / Repository
│   └── ddd-4-rust-core
├── serde/                      ← Jackson-equivalent: AbstractEvent / Serializer / Deserializer
│   └── ddd-4-rust-serde
├── esc/                        ← Event Sourcing Context: EventStore / Repository / StreamId
│   └── ddd-4-rust-esc
├── codegen-api/                ← proc-macro API surface
│   └── ddd-4-rust-codegen-api
├── codegen-processor/          ← proc-macro implementation
│   └── ddd-4-rust-codegen-processor
├── test/                       ← 测试工具与共享 fixtures
│   └── ddd-4-rust-test
└── docs/
    ├── ARCHITECTURE.md         ← 架构与 Java 对照
    ├── IMPLEMENTATION_PLAN.md  ← 实施计划
    └── MIGRATION_STATUS.md     ← 迁移进度
```

### Crate Map / Crate 一览

| Crate | Version | Responsibility | Key Dependencies |
|---|---|---|---|
| `ddd-4-rust-core` | 0.7.0 | Core types / traits / errors / Repository SPI | (none internal) |
| `ddd-4-rust-serde` | 0.7.0 | Serde integration for events | core, serde |
| `ddd-4-rust-esc` | 0.7.0 | Event Store Context (Repository / StreamId) | core, serde |
| `ddd-4-rust-codegen-api` | 0.7.0 | Proc-macro API surface | syn, quote |
| `ddd-4-rust-codegen-processor` | 0.7.0 | Proc-macro implementation | syn, quote, codegen-api |
| `ddd-4-rust-test` | 0.7.0 | Test utilities & shared fixtures | core |

---

## 🚀 Quick Start

### Installation

```toml
# Cargo.toml
[dependencies]
ddd-4-rust-core = "0.7"
serde = { version = "1", features = ["derive"] }
```

> ⚠️ **Not yet published to crates.io.** Until then, use a git/path dependency:
>
> ```toml
> ddd-4-rust-core = { git = "https://github.com/ddd-4-rust/ddd-4-rust", branch = "main" }
> ```

### Define an Aggregate Root

```rust
use ddd_4_rust_core::prelude::*;

#[derive(Debug, Clone, DomainEvent)]
#[domain_event(aggregate = "order")]
pub enum OrderEvent {
    Created { id: OrderId, total: Money },
    Paid    { id: OrderId, at: chrono::DateTime<Utc> },
    Shipped { id: OrderId, tracking: String },
}

#[derive(Debug, AggregateRoot)]
#[aggregate(id = OrderId, event = OrderEvent)]
pub struct Order {
    id: OrderId,
    total: Money,
    status: OrderStatus,
}
```

### Use a Repository

```rust
#[async_trait]
impl Repository<Order, OrderId> for OrderRepository {
    async fn load(&self, id: &OrderId) -> Result<Option<Order>, RepoError> { /* ... */ }
    async fn save(&self, agg: &Order) -> Result<(), RepoError> { /* ... */ }
}
```

### Build & Test

```bash
cargo build --workspace
cargo test  --workspace
cargo doc   --workspace --no-deps --open
```

---

## 🆚 Differences from the Java version / 与 Java 版本的差异

| Aspect | Java (fuinorg/ddd-4-java) | Rust (this port) |
|---|---|---|
| Identity | Generic `Entity<ID>` | Phantom-typed `Entity<TId>` |
| Mutability | Mutable aggregate + replay | Internal event-sourced reconstruction |
| Threading | Synchronous | `async-trait` first, sync fallback |
| Serialization | Jackson modules | Serde with feature flags |
| Annotations | `@ApplyEvent` runtime | `#[apply_event]` proc-macro at compile time |
| Type parameters | `<ID extends AggregateRootId>` | `PhantomData<TId>` zero-cost |
| Error handling | checked exceptions | `Result<T, E>` |

---

## 📊 Migration Status / 迁移进度

> Last updated: 2026-07-21

| crate | Target .rs files | Completed | Completion |
|---|---|---|---|
| `ddd-4-rust-core` | 27 | 17 | 63% |
| `ddd-4-rust-serde` | 9 | 3 | 33% |
| `ddd-4-rust-esc` | 4 | 3 | 75% |
| `ddd-4-rust-codegen-api` | 1 | 1 | 100% |
| `ddd-4-rust-codegen-processor` | 1 | 1(*) | 10% (stub) |
| `ddd-4-rust-test` | 1 | 1 | 100% |
| **Overall** | **~45** | **26** | **~58%** |

Full status: see [`docs/MIGRATION_STATUS.md`](docs/MIGRATION_STATUS.md).

---

## 🗺️ Architecture Mapping / 架构对照

| Java | Rust | Location |
|---|---|---|
| `Event` | `Event` trait | `core` |
| `DomainEvent<ID>` | `DomainEvent<ID>` trait | `core` |
| `EntityId` | `EntityId` trait | `core` |
| `AggregateRootId` | `AggregateRootId` trait | `core` |
| `AggregateRoot<ID>` | `AggregateRoot<ID>` trait | `core` |
| `Repository<ID, T>` | `Repository<ID, T>` trait | `core` |
| `AbstractAggregateRoot<ID>` | `AbstractAggregateRoot<ID>` struct | `core` |
| `ApplyEventHandler` | `ApplyEventHandler` trait | `core` |
| `@ApplyEvent` (Java) | `#[apply_event]` (Rust) | `codegen-processor` |
| `@ChildEntityLocator` | `#[child_locator]` | `codegen-processor` |
| `@HasEntityTypeConstant` | `#[derive(EntityId)]` | `codegen-processor` |
| `AbstractEvent` (Jackson) | `AbstractEvent` struct | `serde` |
| `EventStoreRepository` | `EventStoreRepository` struct | `esc` |

---

## 📚 Related Projects / 相关项目

- 🦀 **[cqrs-4-rust](https://github.com/ddd-4-rust/cqrs-4-rust)** — CQRS building blocks (companion to ddd-4-rust)
- 🧪 **[ddd-cqrs-4-rust-example](https://github.com/ddd-4-rust/ddd-cqrs-4-rust-example)** — Full end-to-end DDD + CQRS + ES example
- 🏛️ **[ddd-4-rust org](https://github.com/ddd-4-rust)** — Parent organization
- ☕ **[fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java)** — Java source (LGPLv3)
- 🏛️ **[ddd-4-java](https://github.com/ddd-4-java)** — Java sibling organization

---

## 📄 License

This Rust port is licensed under **Apache 2.0** — see [LICENSE](LICENSE).

The original Java source from [`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java) is licensed under **LGPLv3**. By the terms of LGPLv3, derivative works may use a different license, but the original must be credited. We do so prominently in the Acknowledgement section above.

---

## 🤝 Contributing / 贡献

Contributions are welcome! Before submitting a PR:

- [ ] Run `cargo fmt --all -- --check`
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] Run `cargo test --workspace`
- [ ] Add unit tests for new public APIs
- [ ] Update relevant docs (CHANGELOG, MIGRATION_STATUS)

---

<div align="center">

**Made with ❤️ by [ddd-4-rust](https://github.com/ddd-4-rust)**
**Ported from [fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java) by Michael Schnell**

</div>