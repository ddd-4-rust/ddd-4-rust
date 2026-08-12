# 依赖版本例外（Conventions / Dependency Exceptions）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **状态**: 截至 2026-07-23，根 `[workspace.dependencies]` 中的直接依赖已通过 Cargo 官方注册表核对；当前锁定版本均为可用最新版，且 MSRV 不高于 1.85。成员 crate 只能使用 `.workspace = true` 继承依赖，禁止各自声明版本。

## 1. 重复 crate 版本默认禁止

以下传递依赖暂时无法由本 Workspace 统一：

| crate | 版本 | 依赖链 | 移除条件 |
|---|---|---|---|
| `syn` | `2.0.119`, `3.0.3` | `chrono -> iana-time-zone -> wasm-bindgen/windows-* -> syn 2`；Workspace proc-macro 使用 `syn 3` | `iana-time-zone` 使用的目标平台宏 crate 升级到 `syn 3` 后移除 |
| `getrandom` | `0.3.4`, `0.4.3` | 仅开发依赖的 `proptest -> rand -> getrandom 0.3`；`uuid -> getrandom 0.4` | `proptest` 的 `rand` 依赖升级到 `getrandom 0.4` 后移除 |

## 2. 工具配置差异

- `cargo-deny` 选取的依赖图不会包含仅开发使用的 `getrandom` 版本对，
  因此 `deny.toml` 只配置 `syn 2.0.119` 例外；
- `cargo tree --workspace --duplicates` 会报告 `getrandom`，仍在此
  保留依赖链和消除条件。

## 3. 升级流程

每次升级重复依赖时：

1. 在 Cargo 官方注册表确认目标版本兼容 MSRV 1.85；
2. 在 `deny.toml` 中调整 `skip` 列表；
3. 删除本文件中的对应行；
4. 运行 `cargo deny check` 与 `cargo tree --workspace --duplicates`
   校验。

## 4. 周期性复核

每季度（1/4/7/10 月）由 `tools/audit_rust_conventions.py` 检查本文件
与 `deny.toml` 一致性；不一致则更新本文件或 `deny.toml`。