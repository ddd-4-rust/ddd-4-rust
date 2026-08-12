# ddd-4-rust Superpowers

> **状态**: 长期文档基线。`ddd-4-java` 0.7.0 → `ddd-4-rust` 0.7.0 的
> 设计与计划已迁移完成，所有"计划 / 规范"内容集中在本目录。
> 机器审计产物保留在 `../migration/`。

## 目录索引

### `specs/` — 已定型的设计规格（长期不变事实）

| 文档 | 作用范围 | 类型 |
|---|---|---|
| [`2026-07-23-ddd-4-rust-workspace-and-layers-design.md`](specs/2026-07-23-ddd-4-rust-workspace-and-layers-design.md) | 根 `Cargo.toml`、8 crate 布局、依赖方向 | 长期事实 |
| [`2026-07-23-java-rust-core-contract-design.md`](specs/2026-07-23-java-rust-core-contract-design.md) | `core` 契约、同步/异步边界、错误建模 | 长期事实 |
| [`2026-07-23-serde-serialization-boundary-design.md`](specs/2026-07-23-serde-serialization-boundary-design.md) | Serde 入口、线格式不变性、自定义适配器 | 长期事实 |
| [`2026-07-23-codegen-proc-macro-design.md`](specs/2026-07-23-codegen-proc-macro-design.md) | `#[apply_event]` / `#[derive(EntityId/DddEvent)]` 契约 | 长期事实 |
| [`2026-07-23-esc-event-store-adapter-design.md`](specs/2026-07-23-esc-event-store-adapter-design.md) | Event Store 适配、缓存、并发、删除 | 长期事实 |
| [`2026-07-23-quality-gates-and-coverage-design.md`](specs/2026-07-23-quality-gates-and-coverage-design.md) | Lints、覆盖率、CI 命令、发布门禁 | 长期事实 |
| [`2026-07-23-web-framework-isolation-design.md`](specs/2026-07-23-web-framework-isolation-design.md) | Axum/Actix 隔离、cqrs 边界 | 长期事实 |

### `plans/` — 实施计划（含完成状态）

| 文档 | 范围 | 状态 |
|---|---|---|
| [`2026-07-22-phase-0-java-1to1-translation.md`](plans/2026-07-22-phase-0-java-1to1-translation.md) | 一对一初始翻译 | ✅ 已完成（`3c0f1f4`） |
| [`2026-07-23-phase-1-core-missing-types.md`](plans/2026-07-23-phase-1-core-missing-types.md) | core 7 个缺失类型 | ✅ 已完成（`2409354`） |
| [`2026-07-23-phase-2-serde-adapters.md`](plans/2026-07-23-phase-2-serde-adapters.md) | serde 自定义适配器 | ✅ 已完成（`955086d`） |
| [`2026-07-23-phase-3-esc-and-codegen-apply-event.md`](plans/2026-07-23-phase-3-esc-and-codegen-apply-event.md) | ESC + `#[apply_event]` | ✅ 已完成（`ee590d3`） |
| [`2026-07-23-phase-4-quality-gates-and-coverage.md`](plans/2026-07-23-phase-4-quality-gates-and-coverage.md) | 覆盖率 100% + 质量门禁 | ✅ 已完成（`8edc31e`+`54f21da`） |
| [`2026-07-23-phase-5-migration-baseline-audit.md`](plans/2026-07-23-phase-5-migration-baseline-audit.md) | 机器审计 + Ktra 发布 | ✅ 已完成（`8edc31e`） |
| [`2026-08-12-phase-6-superpowers-reorg.md`](plans/2026-08-12-phase-6-superpowers-reorg.md) | superpowers 文档重组（本任务） | ✅ 已完成（2026-08-12） |

### `conventions/` — 持续遵守的工程规范

| 文档 | 主题 |
|---|---|
| [`naming.md`](conventions/naming.md) | snake_case / PascalCase / kebab-case / 模块布局 / 文档命名 |
| [`rustdoc.md`](conventions/rustdoc.md) | 中文 rustdoc 强制规范与段标题模板 |
| [`lints.md`](conventions/lints.md) | workspace.lints 严格档与 CI 命令 |
| [`dependency-exceptions.md`](conventions/dependency-exceptions.md) | 重复依赖例外表（syn / getrandom） |

## 1. 命名规范

参考 liteflow 的 `docs/superpowers/` 命名约定：

- 设计规格：`specs/YYYY-MM-DD-<feature>-<scope>-design.md`
- 实施计划：`plans/YYYY-MM-DD-phase-N-<scope>.md` 或
  `YYYY-MM-DD-<scope>.md`（已完成的旧计划沿用此命名）
- 持续规范：`conventions/<topic>.md`
- 索引：`README.md`（位于 `docs/superpowers/`）

## 2. 与 `docs/migration/` 的关系

`docs/migration/` 是 **机器审计产物**，不是计划/规范：

| 目录 | 类型 | 维护方 |
|---|---|---|
| `docs/superpowers/` | 人类设计与计划 | 工程师 + AI 协同 |
| `docs/migration/` | 机器生成的审计快照 | `tools/generate_migration_inventory.py` 等脚本 |

`docs/migration/file_mapping.csv` 是 410/410 映射基线的唯一事实源；
本文档的 spec/plan 引用此 CSV 的统计数字，但不应反向修改它。

## 3. 历史信息溯源

git 历史保留了 9 个 commit 顺序还原：

```text
3c0f1f4  Initial commit: 1:1 Rust translation of ddd-4-java
bc805b6  docs: add bilingual README
3210431  Add docs: IMPLEMENTATION_PLAN, ARCHITECTURE, MIGRATION_STATUS, migration audit
2409354  Phase 1.1: ddd-4-rust-core add 7 missing types
955086d  Phase 1.2: ddd-4-rust-serde add custom adapters
ee590d3  Phase 1.3+1.4: esc AggregateStreamId + codegen-processor #[apply_event]
8edc31e  Complete ddd-4-java 0.7.0 one-to-one migration under crates/ workspace.
54f21da  test(core): 补充核心模块测试覆盖范围
```

每个 Phase 计划文档都标注了对应提交哈希；commit message 与本目录
的计划一一对应。

## 4. 后续规范

新增/修改任何公开 API 时，必须：

1. 在 `specs/` 增加/更新设计规格（如新增 crate / trait / adapter）；
2. 在 `plans/` 写实施计划（即使是单个 PR 也要有阶段化步骤）；
3. 在 `conventions/` 维护持续规范（如新增 lint / 新增命名约定）；
4. 提交前在 `cargo check`、`cargo clippy -D warnings`、
   `cargo test`、`cargo doc -D warnings`、`tools/audit_*.py` 全部通过。

禁止散落 root `.md` 文档；新增内容必须落在 `specs/`、`plans/` 或
`conventions/` 之一。