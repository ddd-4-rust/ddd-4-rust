# Lints 规范（Conventions / Lints）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **状态**: 长期工程规范。

## 1. 根 `[workspace.lints]`

```toml
[workspace.lints.rust]
missing_docs = "deny"
unsafe_code = "forbid"
unused_must_use = "deny"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unwrap_used = "deny"
```

## 2. 不可降级约束

- `unsafe_code = "forbid"`：工作区内禁止 `unsafe` 块（除显式允许的
  第三方依赖）；
- `missing_docs = "deny"`：所有公开项必须有 rustdoc；
- `expect_used` / `panic` / `todo` / `unimplemented` / `unwrap_used` 全 deny；
- Clippy `all` = deny，`pedantic` = warn。

## 3. CI 命令

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo test --workspace --doc --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
```

任一命令失败都阻止合并。

## 4. 允许的 panic / unwrap 场景

仅在以下场景允许（且必须有注释说明）：

1. 编译期 `const_new` 失败（如 `unwrap` 在 `const fn` 中无法表达）；
2. 初始化失败 abort（如 `OnceCell` 失败的进程级 abort）；
3. trybuild 测试夹具（`#[allow(...)]` 局部豁免）。

其他场景必须使用 `Result<T, AggregateError>` 或独立错误类型。

## 5. PR 检查清单

每次修改公开 API 后，必须确认：

- `cargo clippy --workspace -- -D warnings` 通过；
- `cargo test --workspace --doc` 通过；
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` 通过；
- 中文 rustdoc 覆盖率维持 100%（`tools/audit_rust_conventions.py`）。