# ddd-4-java 0.7.0 → ddd-4-rust 迁移状态

冻结基线：`ddd-4-java` 0.7.0，提交 `baa9a989`。本文件只统计严格的一对一映射，Rust 基础设施不计入分母。

> **codegraph 对照结论（2026-07-23）**：双侧核心契约（`AggregateRoot` / `DomainEvent` / `Repository` / `EventStoreRepository` 等）均可解析；`docs/migration/file_mapping.csv` 中 **410/410** 行为 `implemented`，待迁移 **0**。注释语义增强不计入本分母。

## 当前审计结果

| Java 子域 | 映射数 | 已实现并测试 | 待迁移 |
|---|---:|---:|---:|
| core | 94 | 94 | 0 |
| esc | 8 | 8 | 0 |
| jackson | 81 | 81 | 0 |
| jaxb | 76 | 76 | 0 |
| jsonb | 77 | 77 | 0 |
| jsonb-testmodel | 25 | 25 | 0 |
| codegen | 47 | 47 | 0 |
| junit | 1 | 1 | 0 |
| jacoco | 1 | 1 | 0 |
| **合计** | **410** | **410** | **0** |

文件类别基线固定为生产源码 154、测试源码 233、生成源码 3、模板/黄金文件 20。

`jackson` 仍作为冻结 Java 来源子域参与 81 个文件的映射统计；Rust 生产实现已统一为
Serde/serde_json。`json::jackson` 只保留已弃用的迁移兼容门面，主 API 为
`ddd_4_rust_serde::json::serde`。

冻结提交没有 Spring Boot 或 Quarkus 源文件，因此本轮没有 Web adapter
映射，也没有把 Axum/Actix 依赖引入 DDD 基础 Workspace。后续 CQRS/示例
迁移固定采用 Spring Boot → Axum、Quarkus → Actix Web，并放入独立 adapter
crate。

## 校验方式

```bash
python3 tools/generate_migration_inventory.py ../ddd-4-java
python3 tools/audit_migration.py
python3 tools/audit_rust_conventions.py
```

严格审计同时验证 410 行、两侧路径唯一、目标非空、snake_case、模块/Cargo 可达、实现/测试状态以及 332 个 Java 测试场景。扫描 Rust 源树时忽略 `.git` / `target` / `.codegraph`（与 inventory 一致），避免 trybuild 嵌套 `target` 被误报为未登记迁移文件。当前 Core、Serde、ESC 行覆盖率分别为 83.26%、91.18%、83.90%，全 Workspace 为 83.97%。

Rust 规范审计同时验证虚拟 Workspace、8 个成员、Edition 2024、resolver
3、MSRV 1.85、依赖集中继承、无 `mod.rs`、无 glob 公开重导出，以及 448
个非 trybuild 诊断夹具源码文件中的模块、公开对象、字段和方法均包含中文
rustdoc。trybuild 的失败夹具保留固定行号，以确保编译诊断黄金文件稳定。

可发布包通过临时 Ktra 0.7.0 按依赖顺序发布，并已由空白 Cargo 项目从注册表重新下载和 `cargo check`；`test/model` 与 `codegen/example` 保持 `publish = false`。
