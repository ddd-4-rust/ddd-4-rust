# 命名规范（Conventions / Naming）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **状态**: 长期工程规范。所有新增/修改文件必须遵循。

## 1. 源码命名

- crate 目录：简洁的 `core/`、`serde/`、`esc/`；
- 源文件、目录、模块、函数、字段、文件名：`snake_case`；
- Cargo 包名：`kebab-case`；
- 类型、trait、enum、derive 宏：`PascalCase`；
- 关联函数、常量：`SCREAMING_SNAKE_CASE`（如 `ENTITY_TYPE`）。

## 2. 模块布局

- 现代 `foo.rs + foo/` 布局，**禁止** `mod.rs`；
- `lib.rs` 仅承担 crate 文档、模块声明和定向 `pub use`；
- 禁止 glob 公开重导出（`pub use *`）。

## 3. 文档命名

`docs/superpowers/` 子树：

- 设计规格：`specs/YYYY-MM-DD-<feature>-<scope>-design.md`；
- 实施计划：`plans/YYYY-MM-DD-phase-N-<scope>.md`；
- 持续规范：`conventions/<topic>.md`；
- 索引：`README.md`（位于 `docs/superpowers/`）。

`docs/migration/` 机器审计产物：

- `file_mapping.csv`（机器审计唯一事实源，410 行映射）；
- `infrastructure_files.txt`（不计入 410 的 Rust 基础设施清单）；
- `java-tree-full.md` / `rust-tree-full.md`（目录快照）。

## 4. Rust 文件映射规则

- 每个 Java 文件有唯一 Rust 文件映射；
- 禁止在 `exceptions.rs` / `entity_type.rs` 等聚合文件中继续合并多个 Java 文件；
- 统一错误包装、共享解析逻辑、测试入口可放在不计数的私有基础设施文件中；
- 计数规则由 `tools/audit_migration.py` 强制。

## 5. Rustdoc 命名

- 每个公开模块、对象、字段、方法必须有中文 rustdoc；
- rustdoc 段标题使用 `## 职责 / ## 输入输出 / ## 状态变化 / ## 失败语义`；
- 公开 reexport（`pub use foo::Bar`）也需要 rustdoc 说明迁移背景。