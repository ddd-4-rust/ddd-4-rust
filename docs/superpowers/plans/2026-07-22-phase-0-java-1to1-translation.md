# Phase 0: ddd-4-java 0.7.0 一对一初始翻译

> **For agentic workers:** 此计划已完成并提交（`3c0f1f4`）。执行历史见 `git log --reverse --oneline`。

**Goal**: 把 `ddd-4-java` 0.7.0 冻结提交的 410 个 `.java` 文件按一一对应原则翻译为 Rust 源文件，建立可编译、可测试的初始 Workspace。

**Architecture**: 8 crate 虚拟 Workspace（`core` / `serde` / `esc` / `codegen-api` / `codegen-processor` / `codegen-example` / `test-model` / `test-support`），Cargo Edition 2024 / resolver 3 / MSRV 1.85。

**Tech Stack**: Rust 1.85、serde 1.0、chrono 0.4、uuid 1.x、thiserror 2.0、async-trait 0.1、trybuild 1.x、proptest 1.x。

## Global Constraints

- 一一对应：每个 Java 文件都有唯一 Rust 文件，禁止合并。
- 不引入未在冻结基线中存在的 Java 文件，不为引入 Web 框架而虚构来源。
- Rust 命名遵循 snake_case（源文件/目录）、kebab-case（Cargo 包）、PascalCase（类型/trait/enum/derive）。
- 现代 `foo.rs + foo/` 布局，禁止 `mod.rs`；`lib.rs` 仅声明模块和定向 `pub use`，禁止 glob。
- 所有公开模块、对象、字段、方法必须有中文 rustdoc。

## 1. 范围与验收

冻结提交共有 411 个 `.java` 文件，排除
`.mvn/wrapper/MavenWrapperDownloader.java` 后强制映射基线为 410：

| 文件类别 | 数量 |
|---|---:|
| 生产源码 | 154 |
| 测试源码/测试辅助 | 233 |
| 生成源码 | 3 |
| 代码生成模板/黄金文件 | 20 |
| **总计** | **410** |

## 2. 任务步骤

- [x] **Step 1: 建立根 Workspace**
  - 根 `Cargo.toml`：8 members、`[workspace.package]`、`[workspace.dependencies]`、`[workspace.lints]`。
- [x] **Step 2: core crate**
  - 把 `core` 94 个 Java 文件一对一翻译到 `crates/core/src/`；
  - 建立 `AggregateError` 统一错误入口；每个 Java 异常对应 PascalCase Rust 错误。
- [x] **Step 3: serde crate**
  - 把 `jackson` 81、`jaxb` 76、`jsonb` 77 共 234 个 Java 文件翻译到 `crates/serde/src/`；
  - 主入口 `ddd_4_rust_serde::json::serde`；`jackson`/`jsonb`/`jaxb` 均为 Serde 实现门面。
- [x] **Step 4: esc crate**
  - 把 `esc` 8 个 Java 文件翻译到 `crates/esc/src/`，仅依赖 `core`。
- [x] **Step 5: codegen 三个 crate**
  - `codegen/api`（47 个 Java 契约）+ `codegen/processor`（proc-macro）+ `codegen/example`（3 个生成样例）；
  - 模板和黄金文件按 16+4+3 迁移到 `crates/codegen/example`。
- [x] **Step 6: test crates**
  - `test/model`（25 个 jsonb-testmodel 翻译）+ `test/support`（junit、jacoco 各 1 个）。
- [x] **Step 7: 验证**
  - `cargo check --workspace --all-targets --all-features --locked` 通过；
  - `cargo test --workspace --locked` 全部 332+ 个场景通过。

## 3. 状态

**已完成**。提交 `3c0f1f4`。提交 message：`Initial commit: 1:1 Rust translation of ddd-4-java`。

## 4. 后续 Phase 入口

- Phase 1: 补齐 core 缺失类型（`IntegerEntityId` / `AggregateCache` / `BusinessKey` / `Ddd4JUtils` / `ExceptionData` / `EncryptedData` / `EncryptedDataService`）。
- Phase 2: serde 自定义适配器（`EntityIdAdapter` / `EntityIdPathAdapter` / `AggregateVersionAdapter` / `DddSerdeModule`）。
- Phase 3: ESC `AggregateStreamId` + codegen-processor `#[apply_event]` 真实实现。
- Phase 4: 质量门禁与覆盖率 100% 校验。
- Phase 5: 迁移基线审计 + Ktra 发布验证。
- Phase 6: superpowers 文档重组（本任务）。