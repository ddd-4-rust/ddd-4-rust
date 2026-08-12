# Phase 5: 迁移基线审计与 Ktra 发布验证

> **For agentic workers:** 此计划已完成并提交（`8edc31e`）。

**Goal**: 用机器审计工具保证 410/410 一对一映射不漂移；通过临时
Ktra 注册表证明公共 crate 能脱离本地 Workspace 使用。

**Architecture**: 4 个审计脚本 + 1 个 Ktra 验证脚本；审计脚本以
`docs/migration/file_mapping.csv` 为唯一事实源。

**Tech Stack**: Python 3.x、stdlib `csv` / `pathlib` / `argparse`、
`cargo package`、`ktra` 0.x 临时注册表。

## Global Constraints

- 审计脚本任何一条失败都阻止合并；
- Ktra 发布顺序固定为 `core → codegen-api → codegen-processor →
  serde → esc → test-support`；
- 空白 Cargo 项目从 Ktra 拉取版本后必须 `cargo check` 通过；
- `file_mapping.csv` 是唯一事实源；不得用其他临时文件替代。

## 1. 任务步骤

- [x] **Step 1: `generate_migration_inventory.py`**
  - 扫描 `ddd-4-java` 冻结提交，生成 `file_mapping.csv`；
  - 字段：`java_path`、`rust_path`、`kind`、`java_type`、`rust_type`、
  `scenario_count`、`implementation_status`、`test_status`、`wire_fixture`。
- [x] **Step 2: `audit_migration.py`**
  - 校验 410 行、两侧路径唯一、目标非空、snake_case、模块/Cargo 可达、
  `implementation_status == "implemented"`、`test_status` 合法；
  - 校验 332 个 Java 测试场景被覆盖。
- [x] **Step 3: `audit_rust_conventions.py`**
  - 校验虚拟 Workspace、8 个成员、Edition 2024、resolver 3、MSRV 1.85、
  依赖集中继承、无 `mod.rs`、无 glob 公开重导出；
  - 校验中文 rustdoc 覆盖率 100%。
- [x] **Step 4: `check_lcov_full_coverage.py`**
  - 校验 LCOV `DA:0` 为空（按文件可执行行）；
  - 与 `cargo llvm-cov --fail-under-lines 98` 互补。
- [x] **Step 5: `verify_local_registry.sh`**
  - 启动临时 Ktra 注册表，按依赖顺序发布 6 个公共 crate 的 0.7.0；
  - 空白 Cargo 项目从 Ktra 下载并 `cargo check`：通过。
- [x] **Step 6: CI 接入**
  - 4 个审计脚本作为 CI 必过项；
  - `verify_local_registry.sh` 在 release tag 推送时触发。

## 2. 验证

- `python3 tools/generate_migration_inventory.py ../ddd-4-java`：成功；
- `python3 tools/audit_migration.py`：exit 0；
- `python3 tools/audit_rust_conventions.py`：exit 0；
- `python3 tools/check_lcov_full_coverage.py target/lcov.info`：exit 0；
- `bash tools/verify_local_registry.sh`：exit 0。

## 3. 状态

**已完成**。当前实测：

- `file_mapping.csv` 共 410 行，状态全为 `implemented`，待迁移 0；
- `test/model` 与 `codegen/example` 保持 `publish = false`；
- 临时 Ktra 0.7.0 已发布 `core → codegen-api → codegen-processor → serde → esc → test-support`；
- 空白 Cargo 项目从 Ktra 下载并 `cargo check`：通过。

提交：`8edc31e`（Complete ddd-4-java 0.7.0 one-to-one migration under crates/ workspace.）。

## 4. 后续 Phase 入口

- Phase 6：superpowers 文档重组（本任务）。