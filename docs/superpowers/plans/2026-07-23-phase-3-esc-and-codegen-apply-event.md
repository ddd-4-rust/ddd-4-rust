# Phase 1.3+1.4: ESC AggregateStreamId 与 codegen #[apply_event] 真实实现

> **For agentic workers:** 此计划已完成并提交（`ee590d3`）。

**Goal**: 在 `ddd-4-rust-esc` 中实现 `AggregateStreamId` 强类型流 ID，
并在 `ddd-4-rust-codegen-processor` 中实现 `#[apply_event]` 宏的
真实事件分发逻辑（替换占位实现）。

**Architecture**: `AggregateStreamId` 与 core 的 `AggregateRootId` /
`EntityId` 同构；`#[apply_event]` 通过 proc-macro 展开为
`ApplyEventHandler::try_apply_event` 实现，按事件类型分派。

**Tech Stack**: syn 3.0.3、quote 1.0.47、proc-macro2 1.0.107。

## Global Constraints

- proc-macro 展开代码必须可独立 `cargo check` 通过；
- 编译错误必须定位到用户输入 token；
- 中文 rustdoc；trybuild 黄金文件覆盖合法/非法两种情况；
- 不引入新依赖。

## 1. 任务步骤

### 1.1 ESC `AggregateStreamId`

- [x] **Step 1**: 实现 `AggregateStreamId` struct（`String` newtype）；
- [x] **Step 2**: 实现 `From<&AggregateRootId>` / `Display` / `AsRef<str>`；
- [x] **Step 3**: 在 `EventStore::read_stream_by_id` / `append` 中替换字符串参数；
- [x] **Step 4**: 测试：构造、序列化、错误流 ID（空字符串、含特殊字符）。

### 1.2 codegen `#[apply_event]`

- [x] **Step 5**: 实现 `apply_event` proc-macro 入口（解析 `impl` 块）；
- [x] **Step 6**: 为每个 `fn on_xxx(&mut self, e: E) -> Result<...>` 生成
  `try_apply_event` 分支；
- [x] **Step 7**: 不匹配事件返回 `Ok(false)`，由聚合根转换为 `EventHandlerNotFound`；
- [x] **Step 8**: 编写 trybuild 黄金文件（`pass_apply_event.rs` /
  `fail_duplicate_handler.rs` / `fail_unknown_event.rs` 等）；
- [x] **Step 9**: 在 `codegen/example` 中验证展开结果可独立 `cargo check`；
- [x] **Step 10**: 中文 rustdoc 覆盖率 100%。

## 2. 验证

- `cargo test --workspace --all-features --locked`：全部通过；
- `cargo clippy --workspace -- -D warnings`：通过；
- trybuild 全部 pass_* 与 fail_* 案例通过；
- 诊断行号稳定（无意义抖动）。

## 3. 状态

**已完成**。提交 `ee590d3`。提交 message：
`Phase 1.3+1.4: esc AggregateStreamId + codegen-processor #[apply_event] real implementation`。

## 4. 后续 Phase 入口

- Phase 4：质量门禁与覆盖率 100% 校验；
- Phase 5：迁移基线审计 + Ktra 发布验证。