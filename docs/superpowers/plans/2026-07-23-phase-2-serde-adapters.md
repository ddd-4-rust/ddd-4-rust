# Phase 1.2: ddd-4-rust-serde 自定义适配器

> **For agentic workers:** 此计划已完成并提交（`955086d`）。

**Goal**: 在 `ddd-4-rust-serde` 中实现 4 个自定义适配器，让 serde 在
保留 Java 0.7.0 线格式的同时处理 typed ID、聚合版本和领域异常。

**Architecture**: 4 个适配器作为独立模块组织，由 `DddSerdeModule`
聚合为 `serde::Module` 入口；不修改 `serde_json` 默认行为，仅通过
`with_adapter` 启用。

**Tech Stack**: serde 1.0.229、serde_json 1.0.151、`crate::json::serde`
作为主入口。

## Global Constraints

- 适配器不得修改 typed ID / 聚合版本 / 异常数据的线格式；
- `DddSerdeModule` 必须导出独立 trait，不污染 `serde_json` 全局；
- 中文 rustdoc；测试覆盖正/反向序列化与跨语言 JSON 兼容性。

## 1. 任务步骤

- [x] **Step 1: `EntityIdAdapter`**
  - `serialize`: typed ID → `as_string()` 字符串；
  - `deserialize`: 字符串 → typed ID（验证 `is_valid`）。
- [x] **Step 2: `EntityIdPathAdapter`**
  - 实体 ID 路径的多段字符串编码/解码。
- [x] **Step 3: `AggregateVersionAdapter`**
  - `i64` 聚合版本号的人类可读序列化（与 Java 一致）。
- [x] **Step 4: `DddSerdeModule`**
  - 聚合上述适配器的模块；
  - 提供 `register(serde_json::Serializer)` 辅助函数。
- [x] **Step 5: 主入口集成**
  - 在 `crates/serde/src/json/serde.rs` 暴露 `with_ddd_adapters()`
  辅助函数；
  - 默认行为不变，开启适配器后所有 typed ID 自动按线格式序列化。

## 2. 验证

- `cargo test -p ddd-4-rust-serde`：新增 ≥ 4 个适配器测试文件；
- 跨语言测试：`serde_replacement.rs` 加载 ddd-4-java 的 7 个代表性
  JSON 样本并断言反序列化结果与 Java 等价；
- 中文 rustdoc 覆盖率 100%。

## 3. 状态

**已完成**。提交 `955086d`。提交 message：
`Phase 1.2: ddd-4-rust-serde add custom adapters (EntityId, EntityIdPath, AggregateVersion, DddSerdeModule)`。

## 4. 后续 Phase 入口

- Phase 1.3+1.4：esc `AggregateStreamId` + codegen `#[apply_event]`。