# Phase 4: 质量门禁与可执行行覆盖率 100%

> **For agentic workers:** 此计划已完成并提交（`8edc31e` 中的 coverage 部分 + `54f21da`）。

**Goal**: 让 core / serde / esc / 整个 Workspace 的可执行行覆盖率达到
100%，并把质量门禁命令固化为 CI 必过项。

**Architecture**: 通过补充单元测试 + trybuild 黄金文件 + `serde_replacement` 跨语言样本覆盖所有可执行行；由
`tools/check_lcov_full_coverage.py` 强制 `DA:` 无 0 命中。

**Tech Stack**: `cargo llvm-cov` 1.x、`lcov` 工具链、proptest 1.x。

## Global Constraints

- `cargo llvm-cov` 汇总 Lines 因 Rust/LLVM 偏差可低于 100%；CI 以
  `--fail-under-lines 98` 作为该汇总指标底线；
- 同时强制 `tools/check_lcov_full_coverage.py` 通过（按文件 `DA:` 0
  命中为 0）；
- 新增测试不得降低覆盖率；
- 不允许 `.unwrap` / `.expect` / `panic!` / `todo!` / `unimplemented!`
  出现在生产代码。

## 1. 任务步骤

- [x] **Step 1: 全量 baseline**
  - 运行 `cargo llvm-cov --workspace --all-features --lcov`；
  - 记录 core / serde / esc 各自的 line/function coverage。
- [x] **Step 2: 补充 core 测试**
  - 在 `crates/core/src/` 与 `crates/core/tests/` 补充测试，覆盖：
  - 异常类型构造/序列化；
  - 聚合根事件应用/版本推进；
  - 值对象校验/转换。
- [x] **Step 3: 补充 serde 测试**
  - `serde_replacement.rs` 加载 7 个 ddd-4-java JSON 样本；
  - 覆盖 typed ID、聚合版本、异常数据、ZonedDateTime 往返。
- [x] **Step 4: 补充 esc 测试**
  - 内存事件存储的 read/append/conflict/delete/cache 路径。
- [x] **Step 5: 校验 `DA:0` 为空**
  - `tools/check_lcov_full_coverage.py target/lcov.info`：通过；
  - 任何文件 `DA:` 出现 0 命中立即补充测试。
- [x] **Step 6: CI 接入**
  - 把 11 条门禁命令写入 CI（见 `docs/superpowers/specs/2026-07-23-quality-gates-and-coverage-design.md`）。

## 2. 验证

- `cargo llvm-cov --workspace --all-features --lcov --fail-under-lines 98`：通过；
- `tools/check_lcov_full_coverage.py target/lcov.info`：通过；
- 11 条门禁命令全部成功。

## 3. 状态

**已完成**。当前实测：LCOV `DA:0` 为空、函数覆盖 100%；`cargo llvm-cov` 汇总 Lines 约 98.9%（Rust/LLVM 对花括号与 derive/async 区域的已知统计偏差）。

提交链：`8edc31e`（覆盖率达标）+ `54f21da`（test(core): 补充核心模块测试覆盖范围）。

## 4. 后续 Phase 入口

- Phase 5：迁移基线审计 + Ktra 发布验证；
- Phase 6：superpowers 文档重组。