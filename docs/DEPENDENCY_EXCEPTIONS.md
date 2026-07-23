# 依赖版本审计与重复版本例外

截至 2026-07-23，根 `[workspace.dependencies]` 中的直接依赖已通过 Cargo
官方注册表核对；当前锁定版本均为可用最新版，且 MSRV 不高于 1.85。成员
crate 只能使用 `.workspace = true` 继承依赖，禁止各自声明版本。

重复 crate 版本默认禁止。以下传递依赖暂时无法由本 Workspace 统一：

| crate | 版本 | 依赖链 | 移除条件 |
|---|---|---|---|
| `syn` | 2.0.119, 3.0.3 | `chrono -> iana-time-zone -> wasm-bindgen/windows-* -> syn 2`；Workspace proc-macro 使用 `syn 3` | `iana-time-zone` 使用的目标平台宏 crate 升级到 `syn 3` 后移除 |
| `getrandom` | 0.3.4, 0.4.3 | 仅开发依赖的 `proptest -> rand -> getrandom 0.3`；`uuid -> getrandom 0.4` | `proptest` 的 `rand` 依赖升级到 `getrandom 0.4` 后移除 |

`cargo-deny` 选取的依赖图不会包含仅开发使用的 `getrandom` 版本对，因此
`deny.toml` 只配置 `syn 2.0.119` 例外；`cargo tree --workspace
--duplicates` 会报告 `getrandom`，所以仍在此保留依赖链和消除条件。
