# 中文 rustdoc 规范（Conventions / Rustdoc）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **状态**: 长期工程规范。所有新增/修改公开 API 必须遵循。

## 1. 强制约束

- 所有 crate 继承 `missing_docs = "deny"` 和 `broken_intra_doc_links = "deny"`；
- 中文 rustdoc 覆盖率 100%（由 `tools/audit_rust_conventions.py` 强制）；
- 每个公开模块、对象、字段、方法必须有详细中文 rustdoc；
- rustdoc 必须说明：职责、输入输出、状态变化或失败语义。

## 2. 段标题模板

每个公开项使用以下段标题（按需要保留必要项）：

```text
## 职责
（用一两句话说清此项的目的）

## 输入
（参数与外部依赖）

## 输出
（返回值、状态变化、副作用）

## 失败语义
（何时返回 `Err`、何时 panic、错误如何传播）
```

## 3. 示例

```rust
/// 聚合根事件应用处理器 trait。
///
/// ## 职责
/// 根据事件类型分派到对应的 `on_xxx` 方法，记录是否应用成功。
///
/// ## 输入
/// - `event`：要应用的事件引用，类型擦除为 `&dyn Any`。
///
/// ## 输出
/// - `Ok(true)`：事件已应用；
/// - `Ok(false)`：没有处理器，由聚合根转换为 `EventHandlerNotFound`；
/// - `Err(e)`：处理器存在但执行失败，保留具体错误。
///
/// ## 失败语义
/// 不在生产路径使用 `panic!`；调用方负责把 `Ok(false)` 转错误。
pub trait ApplyEventHandler {
    fn try_apply_event(&mut self, event: &dyn std::any::Any) -> Result<bool, AggregateError>;
}
```

## 4. 不允许的写法

- 空 rustdoc（仅 `///` 没有内容）；
- 英文-only rustdoc（覆盖范围不算中文）；
- "TBD"、"TODO" 占位说明；
- 把测试覆盖与 rustdoc 混写（rustdoc 不写测试示例）。

## 5. 集成位置

- 中文 rustdoc 检查由 `tools/audit_rust_conventions.py` 强制；
- CI 必过项；
- 任何 PR 修改公开 API 必须同步更新中文 rustdoc。