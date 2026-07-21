# ddd-4-rust 最终实施计划

> **版本**：v1.0  
> **基线**：ddd-4-java v0.7.0 / cqrs-4-java v0.6.0 / ddd-cqrs-4-java-example v0.5.0  
> **目标仓库**：https://github.com/ddd-4-rust  
> **参考项目**：[hitool-rs](https://github.com/feilongproject/hitool-rs)、[sa-token-rs](https://github.com/feilongproject/sa-token-rs)  
> **当前完成度**：≈ 60-70%（骨架 + 核心 trait 完成，缺 proc-macro 真实现 + serde 适配器 + 测试 + main 入口）  
> **定位**：一比一复刻 ddd-4-java/cqrs-4-java 的 Rust 实现，命名/目录/API 语义严格对齐  
> **状态**：✅ 已批准，执行中

---

## 目录

- [一、项目背景与目标](#一项目背景与目标)
- [二、核心设计原则](#二核心设计原则)
- [三、命名映射规则](#三命名映射规则)
- [四、Workspace 总体结构](#四workspace-总体结构)
- [五、Core 内部模块](#五core-内部模块)
- [六、核心 Trait 签名](#六核心-trait-签名)
- [七、proc-macro 设计](#七proc-macro-设计)
- [八、生态适配层](#八生态适配层)
- [九、关键技术决策](#九关键技术决策)
- [十、分阶段实施计划](#十分阶段实施计划)
- [十一、依赖清单](#十一依赖清单)
- [十二、测试体系](#十二测试体系)
- [十三、Java ↔ Rust 文件对应](#十三java--rust-文件对应)
- [附录 A：与 ddd-4-java 的关键差异](#附录-a与-ddd-4-java-的关键差异)

---

## 一、项目背景与目标

### 1.1 背景

ddd-4-java (https://github.com/fuinorg/ddd-4-java) 是一套轻量级 DDD/CQRS 框架，提供：

| 项目 | 模块数 | 核心能力 |
|---|---|---|
| **ddd-4-java** | 9 | 事件/聚合/实体/仓库/序列化/代码生成 — 核心 DDD 构建块 |
| **cqrs-4-java** | 10 | 命令/查询/视图/事件分发/Spring Boot/Quarkus 集成 |
| **ddd-cqrs-4-java-example** | 2(框架) | 完整示例（Spring Boot + Quarkus），Person 聚合 + EventStoreDB + MariaDB |

**Rust 生态现状**：没有能与之对标的 DDD 框架。各领域碎片化：`actix-web`/`axum`（Web）、`sqlx`（DB）、`serde`（序列化）。DDD 实践者从 Java 转 Rust 时心智模型完全断裂。

### 1.2 目标

**一比一**复刻 ddd-4-java/cqrs-4-java 到 Rust：

| 维度 | 目标 |
|---|---|
| **文件数量** | Java 约 400+ 主文件 → Rust 目标 400+ 主文件（当前 76） |
| **文件路径** | `org.fuin.ddd4j.core.Event` ↔ `ddd_4_rust_core::event::Event` |
| **对象/方法/参数命名** | 100% 对齐（snake_case 转换除外） |
| **方法逻辑** | 核心逻辑对齐，错误处理 Rust 化（`Result<T, E>`） |
| **已有实现** | 不删减，顺着原思路继续补全 |
| **文档注释** | rustdoc 必须标注"原 Java 对应文件/方法" |
| **Rust 生态** | `uuid`/`chrono`/`serde`/`actix-web`/`axum`/`sqlx` |

### 1.3 非目标

- 不追求 Java 字节码级 1:1
- 不复刻 Java 反射（改用 proc-macro 编译期生成）
- 不复刻 Jandex 运行时类路径扫描（改用 `inventory` crate 编译时自动注册）
- 不复刻 JAXB（XML）、Quarkus、jacoco

---

## 二、核心设计原则

借鉴 hitool-rs 和 sa-token-rs 的成熟决策：

### 原则 1：核心 sync + 适配层 async

| 层 | 同步/异步 | 理由 |
|---|---|---|
| `ddd-4-rust-core` | **同步** | 纯逻辑，无 IO |
| `ddd-4-rust-serde` | **同步** | serde 序列化 |
| `ddd-4-rust-esc` | **async**（EventStore trait） | 涉及网络 IO |
| `cqrs-4-rust-core` | 同步 | trait 定义 |
| `cqrs-4-rust-actix/axum` | **async** | Web 框架要求 |

### 原则 2：derive + helper attribute 优于 attribute 宏

```rust
#[derive(DddEvent)]           // 生成 Event trait impl + serde + builder
#[apply_event]                 // 生成 try_apply_event 分发
#[child_locator]               // 注册子实体查找
```

### 原则 3：全局状态 `OnceLock<Arc<T>>`

```rust
static ENTITY_ID_REGISTRY: OnceLock<Arc<EntityIdRegistry>> = OnceLock::new();
```

### 原则 4：错误处理单一 enum

```rust
#[derive(Debug, thiserror::Error)]
pub enum AggregateError {
    #[error("aggregate not found: {entity_type} {entity_id}")]
    AggregateNotFound { entity_type: String, entity_id: String },
    #[error("aggregate version conflict: expected {expected_version}, actual {actual_version}")]
    AggregateVersionConflict { expected_version: u32, actual_version: u32 },
    #[error("aggregate already deleted: {entity_type} {entity_id}")]
    AggregateDeleted { entity_type: String, entity_id: String },
    #[error("aggregate already exists: {entity_type} {entity_id}")]
    AggregateAlreadyExists { entity_type: String, entity_id: String },
    #[error("aggregate version not found: version {version}")]
    AggregateVersionNotFound { version: u32 },
    #[error("entity not found: {entity_type} {entity_id}")]
    EntityNotFound { entity_type: String, entity_id: String },
    #[error("duplicate entity: {entity_type} {entity_id}")]
    DuplicateEntity { entity_type: String, entity_id: String },
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}
```

### 原则 5：依赖方向严格单向

```text
ddd-4-rust-core → (无内部依赖)
ddd-4-rust-serde → ddd-4-rust-core
ddd-4-rust-esc → ddd-4-rust-core + ddd-4-rust-serde

cqrs-4-rust-core → ddd-4-rust-core
cqrs-4-rust-serde → ddd-4-rust-core + ddd-4-rust-serde + cqrs-4-rust-core
cqrs-4-rust-esc → cqrs-4-rust-core + ddd-4-rust-esc
cqrs-4-rust-actix → cqrs-4-rust-core + cqrs-4-rust-esc
cqrs-4-rust-axum → cqrs-4-rust-core + cqrs-4-rust-esc
```

### 原则 6：已有实现不删减

当前 76 个 .rs 文件全部保留，通过增量补全达到 100% 覆盖率。

---

## 三、命名映射规则

### 全局规则

| Java | Rust | 示例 |
|---|---|---|
| 包 `org.fuin.ddd4j.core` | crate + mod `ddd_4_rust_core` | — |
| 类 `AggregateRoot` | trait `AggregateRoot` | PascalCase 保留 |
| `getEventId()` | `event_id()` | getter 去前缀 |
| `setXxx(v)` | `set_xxx(v)` | setter 保留前缀 |
| `isXxx()` / `hasXxx()` | `is_xxx()` / `has_xxx()` | — |
| 接口 `EntityId` | trait `EntityId` / struct `EventId` | — |
| `abstract class AbstractAggregateRoot<ID>` | struct `AbstractAggregateRoot<ID>` | — |
| `@ApplyEvent` | `#[apply_event]` proc-macro | 注解 → 宏 |
| `EventId.java` | `event_id.rs` | snake_case 文件名 |
| `package-info.java` | `mod.rs`（目录级） | — |
| `Result<DATA>` | `CqrsResult<D>` | 泛型单字母 |
| Jackson `AbstractEvent` | serde `AbstractEvent` | 序列化框架映射 |
| Spring `SpringJpaViewManager` | actix `ViewManager` | Web 框架映射 |

### 类型映射

| Java | Rust |
|---|---|
| `String` | `String` / `&str` |
| `UUID` | `uuid::Uuid` |
| `ZonedDateTime` | `chrono::DateTime<Utc>` |
| `int` / `Integer` | `i32` / `u32` |
| `boolean` | `bool` |
| `List<T>` | `Vec<T>` |
| `Set<T>` | `Vec<T>`（有序去重） |
| `void` | `()` |
| `@Nullable T` | `Option<T>` |
| `throw Exception` | `Result<T, AggregateError>` |
| `Serializable` | `Serialize + Deserialize` |

### 必须保留的 DDD 业务动词

| Java | Rust | 说明 |
|---|---|---|
| `apply` | `apply` | DDD 领域术语 |
| `loadFromHistory` | `load_from_history` | 事件溯源术语 |
| `aggregateRoot` | `aggregate_root` | DDD 核心概念 |
| `uncommittedChanges` | `uncommitted_changes` | 事件溯源 |
| `markChangesAsCommitted` | `mark_changes_as_committed` | 事件溯源 |
| `entityIdPath` | `entity_id_path` | DDD 标识 |
| `causationId` | `causation_id` | 事件关联 |

---

## 四、Workspace 总体结构

```text
workspace-ddd4j-boot/
├── ddd-4-rust/                                  # 仓库 1
│   ├── Cargo.toml                               # [workspace]
│   ├── docs/
│   │   ├── IMPLEMENTATION_PLAN.md
│   │   ├── ARCHITECTURE.md
│   │   ├── MIGRATION_STATUS.md
│   │   └── migration/
│   │       ├── java-tree-full.md
│   │       ├── rust-tree-full.md
│   │       ├── project-tree-diff.md
│   │       ├── object-method-matrix.md
│   │       └── TEST_AUDIT_REPORT.md
│   ├── core/            (ddd-4-rust-core)
│   ├── serde/           (ddd-4-rust-serde)
│   ├── esc/             (ddd-4-rust-esc)
│   ├── codegen-api/     (ddd-4-rust-codegen-api)
│   ├── codegen-processor/ (ddd-4-rust-codegen-processor)
│   └── test/            (ddd-4-rust-test)
│
├── cqrs-4-rust/                                 # 仓库 2
│   ├── Cargo.toml
│   ├── docs/
│   │   ├── IMPLEMENTATION_PLAN.md
│   │   ├── ARCHITECTURE.md
│   │   └── MIGRATION_STATUS.md
│   ├── core/            (cqrs-4-rust-core)
│   ├── serde/           (cqrs-4-rust-serde)
│   ├── esc/             (cqrs-4-rust-esc)
│   ├── actix/           (cqrs-4-rust-actix)
│   ├── axum/            (cqrs-4-rust-axum)
│   └── test/            (cqrs-4-rust-test)
│
└── ddd-cqrs-4-rust-example/                     # 仓库 3
    ├── Cargo.toml
    ├── actix/
    │   ├── shared/      (cqrs4r-actix-example-shared)
    │   ├── command/     (cqrs4r-actix-example-command)
    │   └── query/       (cqrs4r-actix-example-query)
    └── axum/
        ├── shared/      (cqrs4r-axum-example-shared)
        ├── command/     (cqrs4r-axum-example-command)
        └── query/       (cqrs4r-axum-example-query)
```

---

## 五、Core 内部模块

### ddd-4-rust-core 文件清单

| 文件 | 状态 | 对应 Java |
|---|---|---|
| `lib.rs` | ✅ | — |
| `event.rs` | ✅ | `Event.java` |
| `domain_event.rs` | ✅ | `DomainEvent.java` |
| `entity.rs` | ✅ | `Entity.java` |
| `entity_id.rs` | ✅ | `EntityId.java` |
| `aggregate_root_id.rs` | ✅ | `AggregateRootId.java` |
| `aggregate_root.rs` | ✅ | `AggregateRoot.java` |
| `repository.rs` | ✅ | `Repository.java` |
| `abstract_aggregate_root.rs` | ✅ | `AbstractAggregateRoot.java` |
| `abstract_entity.rs` | ✅ | `AbstractEntity.java` |
| `aggregate_root_uuid.rs` | ✅ | `AggregateRootUuid.java` |
| `aggregate_version.rs` | ✅ | `AggregateVersion.java` |
| `entity_id_path.rs` | ✅ | `EntityIdPath.java` |
| `entity_id_factory.rs` | ✅ | `EntityIdFactory.java` |
| `entity_type.rs` | ✅ | `EntityType.java` + `StringBasedEntityType.java` |
| `event_id.rs` | ✅ | `EventId.java` |
| `event_type.rs` | ✅ | `EventType.java` |
| `exceptions.rs` | ✅ | 10+ 异常类 |
| `integer_entity_id.rs` | ❌ Phase 1.1 | `IntegerEntityId.java` |
| `aggregate_cache.rs` | ❌ Phase 1.1 | `AggregateCache.java` + `AggregateNoCache.java` |
| `business_key.rs` | ❌ Phase 1.1 | `BusinessKey.java` |
| `ddd_utils.rs` | ❌ Phase 1.1 | `Ddd4JUtils.java` |
| `exception_data.rs` | ❌ Phase 1.1 | `ExceptionData.java` |
| `encrypted_data.rs` | ❌ Phase 1.1 | `EncryptedData.java` |
| `encrypted_data_service.rs` | ❌ Phase 1.1 | `EncryptedDataService.java` |
| `entity_id_registry.rs` | ❌ Phase 2.5 | `JandexEntityIdFactory.java`（inventory 替代） |

### ddd-4-rust-serde 文件清单

| 文件 | 状态 | 对应 Java |
|---|---|---|
| `lib.rs` | ✅ | — |
| `abstract_event.rs` | ✅ | `AbstractEvent.java` |
| `abstract_domain_event.rs` | ✅ | `AbstractDomainEvent.java` |
| `entity_id_adapter.rs` | ❌ Phase 1.2 | `EntityIdJacksonSerializer.java` + `EntityIdJacksonDeserializer.java` |
| `entity_id_path_adapter.rs` | ❌ Phase 1.2 | `EntityIdPathJacksonDeserializer.java` |
| `aggregate_version_adapter.rs` | ❌ Phase 1.2 | `AggregateVersionJacksonSerializer.java` + `AggregateVersionJacksonDeserializer.java` |
| `event_id_adapter.rs` | ❌ Phase 1.2 | `EventIdJacksonSerializer.java` + `EventIdJacksonDeserializer.java` |
| `exception_data.rs` | ❌ Phase 1.2 | 12 个异常 Data 类 |
| `ddd_serde_module.rs` | ❌ Phase 1.2 | `Ddd4JacksonModule.java` |

---

## 六、核心 Trait 签名

### 6.1 Event

```rust
pub trait Event: Any + Send + Sync {
    fn event_id(&self) -> &EventId;
    fn event_type(&self) -> &EventType;
    fn event_timestamp(&self) -> &DateTime<Utc>;
    fn correlation_id(&self) -> Option<&EventId>;
    fn causation_id(&self) -> Option<&EventId>;
}
```

### 6.2 DomainEvent<ID>

```rust
pub trait DomainEvent<ID: EntityId + ?Sized>: Event {
    fn entity_id_path(&self) -> &EntityIdPath;
    fn entity_id(&self) -> &ID;
    fn aggregate_version(&self) -> Option<&AggregateVersion>;
}
```

### 6.3 AggregateRoot<ID>

```rust
pub trait AggregateRoot<ID: AggregateRootId + ?Sized>: Entity<ID> {
    fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>];
    fn mark_changes_as_committed(&mut self);
    fn version(&self) -> i32;
    fn next_version(&self) -> i32;
    fn next_apply_version(&self) -> AggregateVersion;
    fn load_from_history(&mut self, history: &[Box<dyn DomainEvent<dyn EntityId>>]);
    fn apply(&mut self, event: Box<dyn DomainEvent<dyn EntityId>>);
}
```

### 6.4 Repository<ID, T>

```rust
#[async_trait]
pub trait Repository<ID: AggregateRootId + ?Sized, T: AggregateRoot<ID> + Send>: Send + Sync {
    fn aggregate_type(&self) -> &dyn EntityType;
    fn create(&self) -> T;
    async fn read(&self, id: &ID) -> Result<T, AggregateError>;
    async fn read_at_version(&self, id: &ID, version: u32) -> Result<T, AggregateError>;
    async fn update(&self, aggregate: &T) -> Result<(), AggregateError>;
    async fn add(&self, aggregate: &T) -> Result<(), AggregateError>;
    async fn delete(&self, id: &ID, expected_version: u32) -> Result<(), AggregateError>;
}
```

### 6.5 Command / CommandExecutor（cqrs-4-rust-core）

```rust
pub trait Command: Event {}
pub trait CommandExecutor<Ctx, Cmd: Command + ?Sized>: Send + Sync {
    fn command_types(&self) -> Vec<EventType>;
    async fn execute(&self, ctx: &Ctx, cmd: &Cmd) -> Result<(), CommandExecutionError>;
}
```

---

## 七、proc-macro 设计

| Java 注解/处理器 | Rust proc-macro | crate | Phase |
|---|---|---|---|
| `@ApplyEvent` | `#[apply_event]` | codegen-processor | 1.4 |
| `@ChildEntityLocator` | `#[child_locator]` | codegen-processor | 1.4 |
| `@HasEntityTypeConstant` | `#[derive(EntityId)]` | codegen-processor | 2.4 |
| `@HasSerializedDataTypeConstant` | `#[derive(SerializedDataType)]` | codegen-processor | 2.4 |
| `ValueObjectProcessor` | `#[derive(DddEvent)]` | codegen-processor | 2.4 |

```rust
// #[apply_event] 使用示例
#[apply_event]
impl Person {
    fn apply_person_created(&mut self, event: &PersonCreatedEvent) { ... }
    fn apply_person_deleted(&mut self, event: &PersonDeletedEvent) { ... }
}

// 生成:
// impl ApplyEventHandler for Person {
//     fn try_apply_event(&mut self, event: &dyn DomainEvent<dyn EntityId>) -> bool {
//         match event.event_type().as_str() {
//             PersonCreatedEvent::EVENT_TYPE => {
//                 if let Some(e) = (event as &dyn Any).downcast_ref::<PersonCreatedEvent>() {
//                     self.apply_person_created(e); return true;
//                 }
//                 false
//             }
//             PersonDeletedEvent::EVENT_TYPE => {
//                 if let Some(e) = (event as &dyn Any).downcast_ref::<PersonDeletedEvent>() {
//                     self.apply_person_deleted(e); return true;
//                 }
//                 false
//             }
//             _ => false,
//         }
//     }
// }
```

---

## 八、生态适配层

| Java 框架 | Rust 实现 | 状态 |
|---|---|---|
| Jackson（JSON） | serde + serde_json | ✅ |
| JSON-B（JSON） | 合并到 serde | ✅ |
| Spring Boot | actix-web（`cqrs-4-rust-actix`） | ⚠️ 骨架 |
| Quarkus | axum（`cqrs-4-rust-axum`） | ⚠️ 骨架 |
| JPA / Hibernate | sqlx | ❌ 待集成 |
| EventStoreDB gRPC | eventstore gRPC client / raw tonic | ❌ 待集成 |
| Jandex（类路径扫描） | inventory + `#[submit]` | ❌ Phase 2.5 |

---

## 九、关键技术决策

| 决策点 | 方案 | 参考 |
|---|---|---|
| 异步模型 | 核心 sync + 适配层 async | sa-token-rs |
| 全局状态 | `OnceLock<Arc<T>>` | sa-token-rs |
| 错误处理 | 每 crate 独立 enum + thiserror | sa-token-rs |
| 序列化 | serde（统一替代 Jackson/JSON-B） | Rust 生态标准 |
| 编译期扫描 | inventory crate | 替代 Jandex |
| Web 框架 | actix-web + axum 双支持 | hitool-rs 多框架策略 |
| DB | sqlx（不造 ORM） | sa-token-rs 原则 |
| 命名一致性 | 严格对齐 Java，snake_case 转换 | hitool-rs 原则 |
| Edition | 2021 | 稳定 |
| Resolver | 2 | 稳定 |

---

## 十、分阶段实施计划

```text
Phase 1 (v0.8.x) ─── ddd-4-rust 补全
    ├── 1.1: core 补全 10 个缺失类型 ⬅ 当前执行
    ├── 1.2: serde 补全 6 个适配器
    ├── 1.3: esc 补全 AggregateStreamId
    └── 1.4: codegen-processor #[apply_event] + #[child_locator] 真实现

Phase 2 (v0.9.x) ─── cqrs-4-rust 补全 + derive 宏
    ├── 2.1: cqrs-core 补全 ToResultCapable
    ├── 2.2: cqrs-serde 补全 result_serde
    ├── 2.3: cqrs-actix/axum 补全 QryProjectionPosition + CRON 调度
    ├── 2.4: codegen-processor #[derive(DddEvent)] + #[derive(EntityId)]
    └── 2.5: inventory-based EntityIdRegistry

Phase 3 (v0.10.x) ─── Example 补全
    ├── 3.1: actix example main.rs + EventStoreDB + sqlx 集成
    ├── 3.2: axum example main.rs + 路由
    ├── 3.3: GlobalExceptionHandler
    └── 3.4: docker-compose + 集成测试

Phase 4 (v1.0.0) ─── 测试 + 文档 + CI + 发布
    ├── 4.1: 单元测试 + 集成测试 + 覆盖率 ≥ 80%
    ├── 4.2: ARCHITECTURE.md + GUIDE.md
    ├── 4.3: CI/CD（fmt + clippy + test + coverage）
    └── 4.4: crates.io 发布
```

---

## 十一、依赖清单

### ddd-4-rust-core

```toml
[dependencies]
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
async-trait = "0.1"
inventory = "0.3"        # Phase 2.5
```

### cqrs-4-rust-actix

```toml
[dependencies]
actix-web = "4"
actix-rt = "2"
tokio = { version = "1", features = ["full"] }
tokio-cron-scheduler = "0.13"
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
```

---

## 十二、测试体系

| 层 | 工具 | 覆盖率目标 |
|---|---|---|
| 单元测试 | `#[cfg(test)]` + `cargo test` | ≥ 80% |
| 集成测试 | `tests/` + testcontainers（EventStoreDB + MariaDB Docker） | 关键路径 100% |
| CI | GitHub Actions: fmt + clippy(-D warnings) + test + tarpaulin | 每次 PR |

### Java 测试对标

| Java 测试类 | Rust 测试 |
|---|---|
| `EventIdTest.java` | `event_id.rs` 内 `#[cfg(test)] mod tests` |
| `AggregateVersionTest.java` | `aggregate_version.rs` 测试 |
| `EntityIdPathTest.java` | `entity_id_path.rs` 测试 |
| `AbstractAggregateRootTest.java` | `abstract_aggregate_root.rs` 测试 |
| `EventStoreRespositoryTest.java` | `tests/event_store_repository_test.rs` |
| `VendorExampleTest.java` | `tests/serde_roundtrip_test.rs` |

---

## 十三、Java ↔ Rust 文件对应

| Java | Rust | 状态 |
|---|---|---|
| `Event.java` | `event.rs` | ✅ |
| `DomainEvent.java` | `domain_event.rs` | ✅ |
| `EntityId.java` | `entity_id.rs` | ✅ |
| `AggregateRootId.java` | `aggregate_root_id.rs` | ✅ |
| `AggregateRoot.java` | `aggregate_root.rs` | ✅ |
| `Entity.java` | `entity.rs` | ✅ |
| `Repository.java` | `repository.rs` | ✅ |
| `AbstractAggregateRoot.java` | `abstract_aggregate_root.rs` | ✅ |
| `AbstractEntity.java` | `abstract_entity.rs` | ✅ |
| `AggregateRootUuid.java` | `aggregate_root_uuid.rs` | ✅ |
| `AggregateVersion.java` | `aggregate_version.rs` | ✅ |
| `EntityIdPath.java` | `entity_id_path.rs` | ✅ |
| `EntityIdFactory.java` | `entity_id_factory.rs` | ✅ |
| `EntityType.java` | `entity_type.rs` | ✅ |
| `StringBasedEntityType.java` | `entity_type.rs`（合并） | ✅ |
| `EventId.java` | `event_id.rs` | ✅ |
| `EventType.java` | `event_type.rs` | ✅ |
| `ApplyEvent.java` | `abstract_aggregate_root.rs`（ApplyEventHandler） | ✅ |
| `Command.java` | `command.rs` | ✅ |
| `AggregateCommand.java` | `aggregate_command.rs` | ✅ |
| `CommandExecutor.java` | `command_executor.rs` | ✅ |
| `AbstractMultiCommandExecutor.java` | `multi_command_executor.rs` | ✅ |
| `Result.java` | `result.rs` | ✅ |
| `View.java` / `JpaView.java` | `view.rs` + `jpa_view.rs` | ✅ |
| `JpaEventHandler.java` | `jpa_event_handler.rs` | ✅ |
| `CqrsUtils.java` | `cqrs_utils.rs` | ✅ |
| `IntegerEntityId.java` | `integer_entity_id.rs` | ❌ Phase 1.1 |
| `AggregateCache.java` | `aggregate_cache.rs` | ❌ Phase 1.1 |
| `BusinessKey.java` | `business_key.rs` | ❌ Phase 1.1 |
| `Ddd4JUtils.java` | `ddd_utils.rs` | ❌ Phase 1.1 |
| `ExceptionData.java` | `exception_data.rs` | ❌ Phase 1.1 |
| `EncryptedData.java` | `encrypted_data.rs` | ❌ Phase 1.1 |
| `EventStoreRepository.java` | `event_store_repository.rs` | ✅ 骨架 |
| `SpringJpaViewManager.java` | `view_manager.rs`（actix） | ✅ 骨架 |
| `QuarkusJpaViewManager.java` | `view_manager.rs`（axum） | ✅ 骨架 |

---

## 附录 A：与 ddd-4-java 的关键差异

| Java | Rust | 原因 |
|---|---|---|
| 反射 `@ApplyEvent` 方法查找 | proc-macro 编译期生成 match | Rust 无运行时反射 |
| Jandex 类路径扫描 | `inventory` + `#[submit]` | Rust 无运行时注解扫描 |
| Jackson / JSON-B 两套序列化 | 统一 `serde` | Rust 单一序列化框架 |
| Spring Boot / Quarkus 两套 Web | actix-web / axum | Rust 无 Spring/Quarkus |
| JPA / Hibernate ORM | sqlx 直接 SQL | 对齐 JpaView 的直接 SQL 模式 |
| 检查型异常 `throws` | `Result<T, AggregateError>` | Rust 无检查型异常 |
| `extends` 继承 | 组合 + trait delegation | Rust 无类继承 |
| `@Nullable` 注解 | `Option<T>` | Rust 标准 |
| `Snowflake` ID | `uuid::Uuid` v4/v7 | 对齐原实现 |
| `@HasPublicStaticValueOfMethod` | `#[derive(EntityId)]` 生成 `value_of` | proc-macro 替代 |
