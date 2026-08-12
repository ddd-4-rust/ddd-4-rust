# Codegen 与 proc-macro 设计

**日期**: 2026-07-23
**作用范围**: `ddd-4-rust-codegen-{api,processor,example}`。
**类型**: 长期架构事实。

---

## 1. 契约映射

| Java 注解 | Rust 宏 / derive | 编译期职责 |
|---|---|---|
| `@ApplyEvent` | `#[apply_event]` | 生成返回 `Result<bool, AggregateError>` 的事件分发 |
| `@ChildEntityLocator` | `#[child_locator]` | 生成子实体定位注册 |
| `@HasEntityTypeConstant` | `#[derive(EntityId)]` | 生成实体 ID 契约和类型常量 |
| 值对象处理器 | 值对象 derive | 生成校验、转换和构建接口 |
| 事件处理器 | `#[derive(DddEvent)]` | 生成事件及领域事件实现 |

## 2. crate 边界

- `codegen/api` 保存契约：公开宏/derive 入口、trait 抽象、生成代码
  类型签名。**不得**依赖 `syn` / `quote` 等 proc-macro 实现细节。
- `codegen/processor` 是 proc-macro crate，承担全部展开工作：
  - 仅依赖 `codegen/api` 与外部 `syn` / `quote` / `proc-macro2`。
  - 全部展开代码必须可独立 `cargo check` 通过。
- `codegen/example` 是 `publish = false` 的验证 crate：
  - 提供 3 个 `src-gen` 生成样例（值对象、事件、`#[apply_event]`）；
  - 提供 4 个模板文件和 16 个 trybuild 黄金文件；
  - 编译样例必须与生产 crate 一同 `cargo test` 通过。

## 3. trybuild 覆盖

`codegen/processor/tests/trybuild.rs` 必须覆盖：

- 合法展开：`pass_apply_event.rs`、`pass_value_objects.rs`、`pass_generic.rs`；
- 错误签名：`fail_child_locator.rs`、`fail_duplicate_handler.rs`、
  `fail_generated_code.rs`、`fail_invalid_attribute.rs`、
  `fail_invalid_handler.rs`、`fail_unknown_event.rs`。

诊断错误必须：

- 定位到用户输入 token（`Span::call_site` 标记的源行/列）；
- 包含失败原因与建议（不接受裸 "expected X, got Y"）；
- 在黄金文件中保持稳定行号，避免无意义抖动。

## 4. `#[apply_event]` 展开契约

输入：

```rust
#[apply_event]
impl Person {
    fn on_created(&mut self, e: PersonCreatedEvent) -> Result<(), AggregateError> { ... }
    fn on_deleted(&mut self, e: PersonDeletedEvent) -> Result<(), AggregateError> { ... }
}
```

展开结果必须实现 `ApplyEventHandler::try_apply_event`，按事件类型
匹配对应处理函数：

- 匹配 → 调用处理器并返回 `Ok(true)`；
- 不匹配 → 返回 `Ok(false)`，由聚合根转换为 `EventHandlerNotFound`。

不得生成 `panic!` 路径；展开代码必须 `cargo clippy --all -- -D warnings`
通过。

## 5. `#[derive(EntityId)]` 与 `#[derive(DddEvent)]`

两者分别提供：

- `#[derive(EntityId)]`：生成 `EntityId` trait 实现 + `entity_type()`
  关联函数；
- `#[derive(DddEvent)]`：生成 `Event` trait + `DomainEvent<ID>` trait
  + Builder（如需要）+ `event_type()` 关联函数。

不得重复已有 Rust 标准 derive（如 `serde::Serialize`）；通过属性宏
补充 Rust 标准能力，而不是与标准库对抗。

## 6. 错误与诊断约束

所有编译错误必须给出包含以下信息的提示：

- 用户 crate 名称；
- 类型名 / 函数名 / 参数名；
- 失败原因（缺字段、类型不匹配、冲突 ID 等）。

禁止无信息错误（`compile_error!("oops")`）。