# Phase 1.1: ddd-4-rust-core 补齐 7 个缺失类型

> **For agentic workers:** 此计划已完成并提交（`2409354`）。

**Goal**: 在 `ddd-4-rust-core` 中补齐一对一翻译阶段遗漏的 7 个核心类型，确保 core 与 ddd-4-java `core` 子域的功能对齐。

**Architecture**: 仅在 `ddd-4-rust-core` 内部新增模块，不修改 crate 边界；所有新增类型继承 core 现有 trait（如 `EntityId`、`BusinessKey`）。

**Tech Stack**: Rust 1.85、`thiserror 2.0.19`、`async-trait 0.1.91`、`uuid 1.24.0`。

## Global Constraints

- 每个新增类型必须有中文 rustdoc 说明职责、输入输出、状态变化。
- 错误建模遵循 `AggregateError` 统一入口；新增类型失败时通过 `From`
  转 `AggregateError`，不在生产路径用 `panic!`。
- 不引入新依赖；只用 `core` 现有 crate 列表（`uuid` / `chrono` /
  `thiserror` / `async-trait`）。
- 不破坏现有 94 个 core 文件的公共 API。
- 测试场景必须覆盖新增类型的核心分支；优先采用属性测试（`proptest`）。

## 1. 任务步骤

- [x] **Step 1: `IntegerEntityId`**
  - 实现 `EntityId` trait，提供 `i64` 编码的强类型 ID；
  - 测试：序列化/反序列化、相等性、`isValid`。
- [x] **Step 2: `AggregateCache`**
  - 实现 `AggregateCachePolicy` trait，提供并发安全的内存缓存；
  - 配套 `AggregateNoCache`（已存在，确认无回归）。
- [x] **Step 3: `BusinessKey`**
  - 提供业务键（如 `PersonName`）与实体 ID 的双向映射 trait；
  - 测试：相同 key 返回相同 ID、不同 key 返回不同 ID。
- [x] **Step 4: `Ddd4JUtils`**
  - 静态工具方法聚合：UUID 校验、时间格式、字符串规范化等；
  - 测试：覆盖每个工具方法的边界输入。
- [x] **Step 5: `ExceptionData`**
  - 实现异常的线格式数据载体（`data-type` / `message` / `params`）；
  - 测试：嵌套 cause 链、参数类型 `Into<JsonValue>`。
- [x] **Step 6: `EncryptedData`**
  - 实现 GDPR 加密数据载体；密钥 ID / 版本 / 内容类型 / 密文。
- [x] **Step 7: `EncryptedDataService`**
  - 定义加解密 trait；提供 `InMemoryCryptoService` 与 `VaultCryptoService`
    适配接口（实现由 `ddd-cqrs-unit` 提供）。

## 2. 验证

- `cargo check -p ddd-4-rust-core --all-targets --locked`：通过；
- `cargo test -p ddd-4-rust-core --locked`：通过，新增 ≥ 7 个测试文件；
- `cargo clippy -p ddd-4-rust-core -- -D warnings`：通过；
- 中文 rustdoc 覆盖率 100%。

## 3. 状态

**已完成**。提交 `2409354`。提交 message：
`Phase 1.1: ddd-4-rust-core add 7 missing types (IntegerEntityId, AggregateCache, BusinessKey, Ddd4JUtils, ExceptionData, EncryptedData, EncryptedDataService)`。

## 4. 后续 Phase 入口

- Phase 1.2：serde 自定义适配器。
- Phase 1.3+1.4：esc `AggregateStreamId` + codegen `#[apply_event]`。