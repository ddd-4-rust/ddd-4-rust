# ddd-4-rust

> **[`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java) 的 idiomatic Rust 翻译版** —— Rust 的领域驱动设计基础构件库。
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2021-orange)](https://doc.rust-lang.org/edition-guide/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v2-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.7.0-blue)](https://github.com/ddd-4-rust/ddd-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/移植自-fuinorg/ddd--4--java-green?logo=github)](https://github.com/fuinorg/ddd-4-java)
[![Progress](https://img.shields.io/badge/迁移进度-58%25-yellow)](docs/MIGRATION_STATUS.md)

---

## 🙏 致谢 — Java 源项目

本项目是以下项目的**一比一 Rust 翻译**：

> **[`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java)**
> 原作者：[Michael Schnell / fuinorg](https://github.com/fuinorg)
> *"Base classes for Domain Driven Design (DDD) with Java"*
>
> 📜 原 Java 源码许可证：**LGPLv3**
> 📌 源项目版本基线：**0.7.0**

**原项目的架构设计、API 形态和领域驱动设计洞见**全部归功于 **`fuinorg`** 项目及其贡献者。本 Rust 移植版的目的是将这套久经考验的 DDD 原语带入 Rust 生态，并采用 **idiomatic 类型安全、所有权语义和 async-first 设计**。

| | Java（源） | Rust（本移植版） |
|---|---|---|
| **仓库** | [fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java) | [ddd-4-rust/ddd-4-rust](https://github.com/ddd-4-rust/ddd-4-rust) |
| **所有者** | [fuinorg](https://github.com/fuinorg) | [ddd-4-rust 组织](https://github.com/ddd-4-rust) |
| **维护者** | Michael Schnell | hiwepy |
| **语言** | Java 17 | Rust 2021 |
| **许可证** | LGPLv3 | Apache 2.0 |
| **版本基线** | 0.7.0 | 0.7.0 |
| **API 兼容性** | — | 1:1 功能等价（必要时采用 idiomatic 写法） |

---

## 🎯 项目定位

**`ddd-4-rust`** 是一个 Cargo workspace，提供 Rust 语言的 **DDD 基础构件**：

- **`Entity<TId>`** — 强类型的实体根
- **`AggregateRoot<TId, TEvent>`** — 支持内部事件溯源的聚合根
- **`DomainEvent`** — 所有领域事件的基 trait
- **`EntityId<TId>`** — newtype 风格的 ID，支持序列化
- **`Repository<T, TId>`** trait — 持久化无关的存储 SPI
- **`#[apply_event]`** — 生成 `try_apply_event` 分发的 proc-macro
- **`AggregateStreamId`** — 事件存储的流标识符

充分利用 Rust 的类型系统：**newtype 模式、trait 约束、PhantomData** 让无效的领域状态在编译期就不可表达。

---

## 🧱 Workspace 结构

```text
ddd-4-rust/                     ← Cargo Workspace (resolver = 2)
├── core/                       ← 核心：Event / DomainEvent / AggregateRoot / EntityId / Repository
│   └── ddd-4-rust-core
├── serde/                      ← Jackson 等价物：AbstractEvent / Serializer / Deserializer
│   └── ddd-4-rust-serde
├── esc/                        ← 事件溯源上下文：EventStore / Repository / StreamId
│   └── ddd-4-rust-esc
├── codegen-api/                ← proc-macro API surface
│   └── ddd-4-rust-codegen-api
├── codegen-processor/          ← proc-macro 实现
│   └── ddd-4-rust-codegen-processor
├── test/                       ← 测试工具与共享 fixtures
│   └── ddd-4-rust-test
└── docs/
    ├── ARCHITECTURE.md         ← 架构与 Java 对照
    ├── IMPLEMENTATION_PLAN.md  ← 实施计划
    └── MIGRATION_STATUS.md     ← 迁移进度
```

### Crate 一览

| Crate | 版本 | 职责 | 关键依赖 |
|---|---|---|---|
| `ddd-4-rust-core` | 0.7.0 | 核心类型 / trait / 错误 / Repository SPI | （无内部依赖） |
| `ddd-4-rust-serde` | 0.7.0 | 事件的 Serde 集成 | core, serde |
| `ddd-4-rust-esc` | 0.7.0 | 事件存储上下文（Repository / StreamId） | core, serde |
| `ddd-4-rust-codegen-api` | 0.7.0 | proc-macro API 表面 | syn, quote |
| `ddd-4-rust-codegen-processor` | 0.7.0 | proc-macro 实现 | syn, quote, codegen-api |
| `ddd-4-rust-test` | 0.7.0 | 测试工具与共享 fixtures | core |

---

## 🚀 快速开始

### 安装

```toml
# Cargo.toml
[dependencies]
ddd-4-rust-core = "0.7"
serde = { version = "1", features = ["derive"] }
```

> ⚠️ **尚未发布到 crates.io。** 在发布之前，使用 git/path 依赖：
>
> ```toml
> ddd-4-rust-core = { git = "https://github.com/ddd-4-rust/ddd-4-rust", branch = "main" }
> ```

### 定义聚合根

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

### 使用 Repository

```rust
#[async_trait]
impl Repository<Order, OrderId> for OrderRepository {
    async fn load(&self, id: &OrderId) -> Result<Option<Order>, RepoError> { /* ... */ }
    async fn save(&self, agg: &Order) -> Result<(), RepoError> { /* ... */ }
}
```

### 构建与测试

```bash
cargo build --workspace
cargo test  --workspace
cargo doc   --workspace --no-deps --open
```

---

## 🆚 与 Java 版本的差异

| 维度 | Java (fuinorg/ddd-4-java) | Rust (本移植版) |
|---|---|---|
| 标识 | 泛型 `Entity<ID>` | Phantom 类型 `Entity<TId>` |
| 可变性 | 可变聚合 + 重放 | 内部事件溯源重建 |
| 线程模型 | 同步 | `async-trait` 优先，同步 fallback |
| 序列化 | Jackson 模块 | Serde feature flags |
| 注解 | 运行时 `@ApplyEvent` | 编译期 `#[apply_event]` proc-macro |
| 类型参数 | `<ID extends AggregateRootId>` | `PhantomData<TId>` 零成本 |
| 错误处理 | checked exceptions | `Result<T, E>` |

---

## 📊 迁移进度

> 最后更新：2026-07-21

| crate | 目标 .rs 文件 | 已完成 | 完成率 |
|---|---|---|---|
| `ddd-4-rust-core` | 27 | 17 | 63% |
| `ddd-4-rust-serde` | 9 | 3 | 33% |
| `ddd-4-rust-esc` | 4 | 3 | 75% |
| `ddd-4-rust-codegen-api` | 1 | 1 | 100% |
| `ddd-4-rust-codegen-processor` | 1 | 1(*) | 10%（stub） |
| `ddd-4-rust-test` | 1 | 1 | 100% |
| **总计** | **~45** | **26** | **~58%** |

完整进度：[`docs/MIGRATION_STATUS.md`](docs/MIGRATION_STATUS.md)

---

## 🗺️ 架构对照

| Java | Rust | 位置 |
|---|---|---|
| `Event` | `Event` trait | `core` |
| `DomainEvent<ID>` | `DomainEvent<ID>` trait | `core` |
| `EntityId` | `EntityId` trait | `core` |
| `AggregateRootId` | `AggregateRootId` trait | `core` |
| `AggregateRoot<ID>` | `AggregateRoot<ID>` trait | `core` |
| `Repository<ID, T>` | `Repository<ID, T>` trait | `core` |
| `AbstractAggregateRoot<ID>` | `AbstractAggregateRoot<ID>` struct | `core` |
| `ApplyEventHandler` | `ApplyEventHandler` trait | `core` |
| `@ApplyEvent`（Java） | `#[apply_event]`（Rust） | `codegen-processor` |
| `@ChildEntityLocator` | `#[child_locator]` | `codegen-processor` |
| `@HasEntityTypeConstant` | `#[derive(EntityId)]` | `codegen-processor` |
| `AbstractEvent`（Jackson） | `AbstractEvent` struct | `serde` |
| `EventStoreRepository` | `EventStoreRepository` struct | `esc` |

---

## 📚 相关项目

- 🦀 **[cqrs-4-rust](https://github.com/ddd-4-rust/cqrs-4-rust)** — CQRS 基础构件（ddd-4-rust 的配套）
- 🧪 **[ddd-cqrs-4-rust-example](https://github.com/ddd-4-rust/ddd-cqrs-4-rust-example)** — DDD + CQRS + ES 完整端到端示例
- 🏛️ **[ddd-4-rust 组织](https://github.com/ddd-4-rust)** — 父组织
- ☕ **[fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java)** — Java 源项目（LGPLv3）
- 🏛️ **[ddd-4-java](https://github.com/ddd-4-java)** — Java 兄弟组织

---

## 📄 许可证

本 Rust 移植版采用 **Apache 2.0** 许可证 —— 见 [LICENSE](LICENSE)。

原 Java 源（来自 [`fuinorg/ddd-4-java`](https://github.com/fuinorg/ddd-4-java)）采用 **LGPLv3**。根据 LGPLv3 的条款，衍生作品可以使用不同的许可证，但必须明确标注原始来源。我们在上面的"致谢"部分显著地做了这一点。

---

## 🤝 贡献

欢迎贡献！提交 PR 前请确保：

- [ ] 运行 `cargo fmt --all -- --check`
- [ ] 运行 `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] 运行 `cargo test --workspace`
- [ ] 为新公共 API 添加单元测试
- [ ] 更新相关文档（CHANGELOG、MIGRATION_STATUS）

---

<div align="center">

**由 [ddd-4-rust](https://github.com/ddd-4-rust) 用 ❤️ 制作**
**移植自 Michael Schnell 的 [fuinorg/ddd-4-java](https://github.com/fuinorg/ddd-4-java)**

</div>