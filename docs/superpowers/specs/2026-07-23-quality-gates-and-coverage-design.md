# 质量门禁与覆盖率设计

**日期**: 2026-07-23
**作用范围**: 全 Workspace、CI、发布流程。
**类型**: 长期架构事实。

---

## 1. 不可降级约束

以下规则由根 `[workspace.lints]` 与审计脚本强制，**不得绕过**：

- `unsafe_code = "forbid"`；
- `missing_docs = "deny"`、`broken_intra_doc_links = "deny"`；
- `unused_must_use = "deny"`；
- Clippy `all` = deny、`pedantic` = warn；
- `expect_used` / `panic` / `todo` / `unimplemented` / `unwrap_used` 全 deny。

生产代码禁止无说明的 `unwrap` / `expect` / `panic` / `todo` /
`unimplemented`。允许场景仅限编译期断言、初始化失败 abort，且必须
在注释中说明。

## 2. 中文 rustdoc

- 每个公开模块、对象、字段、方法必须有详细中文 rustdoc；
- rustdoc 必须说明职责、输入输出、状态变化或失败语义；
- 英文 rustdoc 不要求；
- 双语 README（根目录 `README.md` / `README.zh-CN.md`）按现有约定保留。

## 3. 测试覆盖

- 每个 Java 测试/辅助文件都有唯一 Rust 映射文件；
- 全部 410 个映射文件均纳入 `cargo test` 入口；
- Rust 场景计数不低于 Java 基线的 332 个 `@Test` 场景；
- Core / Serde / ESC / 整个 Workspace 的可执行行覆盖率为 **100%**
  （LCOV `DA:` 无 0 命中；由 `tools/check_lcov_full_coverage.py` 强制）；
- `cargo llvm-cov` 汇总 Lines 因 Rust/LLVM 对花括号与 derive/async
  区域的统计偏差可略低于 100%；CI 以 `--fail-under-lines 98` 作为
  该汇总指标底线。

## 4. CI 门禁顺序

```bash
python3 tools/audit_rust_conventions.py
python3 tools/audit_migration.py
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo test --workspace --doc --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
cargo llvm-cov --workspace --all-features --lcov --output-path target/lcov.info --fail-under-lines 98
python3 tools/check_lcov_full_coverage.py target/lcov.info
cargo deny check
cargo tree --workspace --duplicates
```

任一命令失败则阻止合并。

## 5. 发布门禁

- 所有可发布 crate 必须通过 `cargo package` 内容检查；
- CI **不**向 crates.io 发布，而是启动临时 Ktra 注册表，按
  `core → codegen-api → codegen-processor → serde → esc → test-support`
  发布 0.7.0；
- 空白 Cargo 项目从该注册表下载并执行 `cargo check`，证明包能够
  脱离本地 Workspace 使用。

## 6. 依赖例外

不可消除的重复依赖及移除条件记录在
`docs/superpowers/conventions/dependency-exceptions.md`。当前已知：

| crate | 版本 | 依赖链 | 移除条件 |
|---|---|---|---|
| `syn` | `2.0.119`, `3.0.3` | `chrono -> iana-time-zone -> wasm-bindgen/windows-* -> syn 2`；Workspace proc-macro 使用 `syn 3` | `iana-time-zone` 使用的目标平台宏 crate 升级到 `syn 3` 后移除 |
| `getrandom` | `0.3.4`, `0.4.3` | 仅开发依赖的 `proptest -> rand -> getrandom 0.3`；`uuid -> getrandom 0.4` | `proptest` 的 `rand` 依赖升级到 `getrandom 0.4` 后移除 |

`cargo-deny` 选取的依赖图不会包含仅开发使用的 `getrandom` 版本对，
因此 `deny.toml` 只配置 `syn 2.0.119` 例外；`cargo tree --workspace
--duplicates` 会报告 `getrandom`，仍保留依赖链和消除条件。