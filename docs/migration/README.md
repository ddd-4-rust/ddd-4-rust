# ddd-4-rust 迁移审计产物

> **状态**: 机器审计快照，由 `tools/generate_migration_inventory.py` /
> `audit_migration.py` / `audit_rust_conventions.py` 生成。
> 每次基线变更需重新生成。

## 1. 文件清单

| 文件 | 内容 | 来源脚本 |
|---|---|---|
| `file_mapping.csv` | 410 行一对一映射表（java_path / rust_path / kind / java_type / rust_type / scenario_count / implementation_status / test_status / wire_fixture） | `tools/generate_migration_inventory.py ../ddd-4-java` |
| `infrastructure_files.txt` | 不计入 410 的 Rust 基础设施文件清单 | 同上 |
| `java-tree-full.md` | `ddd-4-java` 冻结提交的目录快照 | 同上 |
| `rust-tree-full.md` | `crates/` 目录快照 | 同上 |

## 2. 与 `docs/superpowers/` 的关系

`docs/superpowers/` 是 **人类设计与计划**；本目录是 **机器审计产物**。
两层之间是 *事实 ↔ 规范* 的层级关系：

- `superpowers/specs/` 引用 `file_mapping.csv` 的统计数字（如"94 个
  core 文件已完成"）作为已确立事实；
- `superpowers/plans/` 描述如何达到这个事实；
- 修改 `file_mapping.csv` 必须同步更新相关 spec/plan；反之亦然。

任何 spec/plan 不得反向修改 `file_mapping.csv`；如发现 CSV 与 spec/
plan 不一致，以 CSV 为准并修正 spec/plan。

## 3. 校验命令

```bash
python3 tools/generate_migration_inventory.py ../ddd-4-java
python3 tools/audit_migration.py
python3 tools/audit_rust_conventions.py
python3 tools/check_lcov_full_coverage.py target/lcov.info
```

期望输出：

- `file_mapping.csv` 共 410 行，`implementation_status` 全为 `implemented`；
- Java 测试场景数 ≥ 332；
- 中文 rustdoc 覆盖率 100%；
- LCOV `DA:0` 为空。

## 4. 冻结基线

- Java 基线：`ddd-4-java` 0.7.0，提交 `baa9a989`；
- Rust 基线：`ddd-4-rust` 0.7.0；
- 计数固定为生产源码 154、测试源码 233、生成源码 3、模板/黄金文件 20。

未来基线升级（如 `ddd-4-java` 0.8.0）必须新增独立 Phase 计划，且
不修改现有 410 行的统计。