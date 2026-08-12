# Phase 6: superpowers 文档重组

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal**: 把 `ddd-4-rust` 的所有"计划 + 规范"内容从散落 root 文档（`ARCHITECTURE.md` / `IMPLEMENTATION_PLAN.md` / `MIGRATION_STATUS.md` / `DEPENDENCY_EXCEPTIONS.md`）按 liteflow superpowers 命名规范重组到 `docs/superpowers/{specs,plans,conventions}/`；删除重复内容；保证一切历史计划与恒定规范可被唯一路径访问。

**Architecture**: 引入 `docs/superpowers/` 子树，沿用 liteflow 命名规范：

- `specs/YYYY-MM-DD-<feature>-<scope>-design.md` — 已定型的设计规格（不变事实）；
- `plans/YYYY-MM-DD-phase-N-<scope>.md` — 实施计划（带完成状态）；
- `conventions/<topic>.md` — 持续遵守的工程规范；
- 根 `docs/superpowers/README.md` 提供索引。

**Tech Stack**: 无新增依赖；纯文档与目录结构变更。

## Global Constraints

- 严格遵循 liteflow `docs/superpowers/` 命名规范；
- 根 `docs/` 不再保留任何"计划/规范"性质的 `.md`，仅保留 `migration/` 机器审计产物；
- `docs/migration/file_mapping.csv` 仍是机器审计唯一事实源；
- 删除前确认每个新文件内容已包含旧文件的全部信息。

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 把 4 个 root `.md` + `DEPENDENCY_EXCEPTIONS.md` 的内容按主题拆解到 superpowers 子树；
2. 命名采用 `YYYY-MM-DD-<scope>-design.md` / `YYYY-MM-DD-phase-N-<scope>.md`；
3. 保留所有已完成阶段（Phase 0–5）作为带状态的计划文档；
4. 把"恒定规范"集中到 `conventions/` 子目录；
5. 删除已被吸收到 superpowers 的旧 root 文档；
6. 保留 `docs/migration/` 机器审计产物不删除。

### 1.2 非目标

- 不修改 `file_mapping.csv` 与审计脚本；
- 不引入新的 CI 步骤（仅文档整理）；
- 不重写 `cargo` workspace 与 crate 边界；
- 不为后续 `cqrs-4-rust` 创建新文档。

## 2. 目标目录布局

```text
docs/
├── superpowers/
│   ├── README.md                                       # 索引
│   ├── specs/
│   │   ├── 2026-07-23-ddd-4-rust-workspace-and-layers-design.md
│   │   ├── 2026-07-23-java-rust-core-contract-design.md
│   │   ├── 2026-07-23-serde-serialization-boundary-design.md
│   │   ├── 2026-07-23-codegen-proc-macro-design.md
│   │   ├── 2026-07-23-esc-event-store-adapter-design.md
│   │   ├── 2026-07-23-quality-gates-and-coverage-design.md
│   │   └── 2026-07-23-web-framework-isolation-design.md
│   ├── plans/
│   │   ├── 2026-07-22-phase-0-java-1to1-translation.md
│   │   ├── 2026-07-23-phase-1-core-missing-types.md
│   │   ├── 2026-07-23-phase-2-serde-adapters.md
│   │   ├── 2026-07-23-phase-3-esc-and-codegen-apply-event.md
│   │   ├── 2026-07-23-phase-4-quality-gates-and-coverage.md
│   │   ├── 2026-07-23-phase-5-migration-baseline-audit.md
│   │   └── 2026-08-12-phase-6-superpowers-reorg.md      # 本文件
│   └── conventions/
│       ├── naming.md
│       ├── rustdoc.md
│       ├── lints.md
│       └── dependency-exceptions.md
└── migration/
    ├── file_mapping.csv                                # 不变
    ├── infrastructure_files.txt                        # 不变
    ├── java-tree-full.md                               # 不变
    ├── rust-tree-full.md                               # 不变
    └── README.md                                       # 新增：说明 superpowers 边界
```

## 3. 文件职责总览

| 旧文件 | 新归宿 |
|---|---|
| `docs/ARCHITECTURE.md`（一、二、三、四、五、六、七章）| `specs/2026-07-23-ddd-4-rust-workspace-and-layers-design.md` + 6 个专题 spec |
| `docs/IMPLEMENTATION_PLAN.md`（一、二、三、四、五、六、七、八章）| `specs/2026-07-23-quality-gates-and-coverage-design.md` + `conventions/` |
| `docs/MIGRATION_STATUS.md` | `plans/2026-07-23-phase-5-migration-baseline-audit.md` 状态段 |
| `docs/DEPENDENCY_EXCEPTIONS.md` | `conventions/dependency-exceptions.md` + `specs/2026-07-23-quality-gates-and-coverage-design.md` §6 |
| git history（`3c0f1f4`/`bc805b6`/`2409354`/`955086d`/`ee590d3`/`3210431`/`cac9e3f`/`8edc31e`/`54f21da`）| `plans/2026-07-22-phase-0..phase-5` 各计划 |

## 4. 任务步骤

- [x] **Step 1: 建立目录骨架**
  ```bash
  mkdir -p docs/superpowers/specs docs/superpowers/plans docs/superpowers/conventions
  ```
- [x] **Step 2: 写 7 个 specs**
  - workspace-and-layers、java-rust-core-contract、serde-serialization-boundary、
    codegen-proc-macro、esc-event-store-adapter、quality-gates-and-coverage、
    web-framework-isolation。
- [x] **Step 3: 写 6 个历史 plan + 1 个本任务 plan**
  - phase-0..phase-5（已完成）+ phase-6（本任务）。
- [x] **Step 4: 写 3 个 conventions + dependency-exceptions**
- [x] **Step 5: 写 `docs/superpowers/README.md` 索引**
- [x] **Step 6: 写 `docs/migration/README.md`**
  - 解释 `file_mapping.csv` 是机器审计唯一事实源；
  - 声明它与 superpowers 文档的关系（事实 ↔ 规范的层级）。
- [x] **Step 7: 删除旧 root 文档**
  - `git rm docs/ARCHITECTURE.md docs/IMPLEMENTATION_PLAN.md docs/MIGRATION_STATUS.md docs/DEPENDENCY_EXCEPTIONS.md`。
- [x] **Step 8: 校验**
  - `find docs -maxdepth 4 -type f -name '*.md'`：无散落 root 规范文档；
  - `git status --short`：仅新增 + 删除，无意外修改。

## 5. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| 旧 root 文档全部吸收 | `ls docs/*.md` 仅剩 `migration/README.md` + `superpowers/` |
| 命名规范 | `find docs/superpowers -name '*.md'` 文件名全部 `YYYY-MM-DD-*.md` 或 `conventions/<topic>.md` |
| 历史可追溯 | `plans/` 含 phase-0..phase-5 共 6 个计划 |
| 当前任务归档 | `plans/2026-08-12-phase-6-superpowers-reorg.md` 存在 |
| 恒定规范可被唯一路径访问 | `conventions/{naming,rustdoc,lints,dependency-exceptions}.md` 全部存在 |
| 索引可读 | `docs/superpowers/README.md` 列出 specs/plans/conventions 入口 |
| `migration/` 边界明确 | `docs/migration/README.md` 存在并解释与 superpowers 的关系 |

## 6. 实施顺序与发布边界

- 不创建 git 分支，单次提交；
- 不修改 Cargo、CI、审计脚本；
- 后续 `cqrs-4-rust` 与 Web adapter 文档另立 spec/plan。

## 7. 后续独立规格

- `cqrs-4-rust` 工作区分层与依赖方向设计（CQRS 阶段启动时创建）。
- `ddd-4-rust` 后续增量（如新事件类型、新适配器、新 proc-macro）按
  "新增 spec → 写 plan → 实施 → 验收"四步走，禁止散落文档。

---

**状态**：本计划描述的是"已经完成"的重组任务；保留作为本次重构的
实施档案，结构与命名遵循 liteflow superpowers 规范。