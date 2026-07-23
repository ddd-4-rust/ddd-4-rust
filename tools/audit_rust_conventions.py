#!/usr/bin/env python3
"""审计 Rust 模块布局、命名和中文 rustdoc 约束。"""

from __future__ import annotations

import re
import tomllib
from pathlib import Path

from add_chinese_rustdoc import (
    CONSTANT_DECLARATION,
    FUNCTION_DECLARATION,
    HAN,
    PUBLIC_FIELD,
    PUBLIC_MODULE,
    TYPE_DECLARATION,
    has_nearby_chinese_doc,
)


ROOT = Path(__file__).resolve().parents[1]
SNAKE_COMPONENT = re.compile(r"^[a-z][a-z0-9_]*$")
PASCAL_CASE = re.compile(r"^[A-Z][A-Za-z0-9]*$")
GLOB_REEXPORT = re.compile(r"\bpub\s+use\s+[^;]*::\s*\*")
KEBAB_COMPONENT = re.compile(r"^[a-z][a-z0-9-]*$")
EXPECTED_MEMBERS = {
    "crates/core",
    "crates/serde",
    "crates/esc",
    "crates/codegen/api",
    "crates/codegen/processor",
    "crates/codegen/example",
    "crates/test/model",
    "crates/test/support",
}


def audit_workspace(errors: list[str]) -> None:
    """校验虚拟 Workspace、成员清单和依赖继承规则。

    根清单集中管理版本与质量策略；成员清单不得重新声明版本，以免不同
    crate 在不知情的情况下产生版本漂移。
    """
    root_manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    if "package" in root_manifest:
        errors.append("根 Cargo.toml 必须是虚拟 Workspace，不能声明 [package]")

    workspace = root_manifest.get("workspace", {})
    if workspace.get("resolver") != "3":
        errors.append("Workspace resolver 必须为 3")
    members = set(workspace.get("members", []))
    if members != EXPECTED_MEMBERS:
        errors.append(
            "Workspace 成员不符合约定："
            f"缺少 {sorted(EXPECTED_MEMBERS - members)}，"
            f"多出 {sorted(members - EXPECTED_MEMBERS)}"
        )

    package_defaults = workspace.get("package", {})
    if package_defaults.get("edition") != "2024":
        errors.append("Workspace Edition 必须为 2024")
    if package_defaults.get("rust-version") != "1.85":
        errors.append("Workspace MSRV 必须为 1.85")

    for member in sorted(EXPECTED_MEMBERS):
        manifest_path = ROOT / member / "Cargo.toml"
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
        package = manifest.get("package", {})
        package_name = package.get("name", "")
        if not KEBAB_COMPONENT.fullmatch(package_name):
            errors.append(f"Cargo 包名不是 kebab-case：{member}:{package_name}")
        for inherited_key in (
            "version",
            "edition",
            "license",
            "authors",
            "repository",
            "rust-version",
        ):
            if package.get(inherited_key, {}).get("workspace") is not True:
                errors.append(
                    f"成员未继承 workspace.package.{inherited_key}：{member}"
                )
        if manifest.get("lints", {}).get("workspace") is not True:
            errors.append(f"成员未继承 workspace.lints：{member}")

        for table_name in ("dependencies", "dev-dependencies", "build-dependencies"):
            for dependency, declaration in manifest.get(table_name, {}).items():
                if isinstance(declaration, str):
                    errors.append(
                        f"成员依赖必须从 workspace 继承：{member}:{table_name}:{dependency}"
                    )
                    continue
                if "version" in declaration:
                    errors.append(
                        f"成员依赖禁止单独声明版本：{member}:{table_name}:{dependency}"
                    )
                if declaration.get("workspace") is not True and "path" not in declaration:
                    errors.append(
                        f"成员依赖未从 workspace 继承："
                        f"{member}:{table_name}:{dependency}"
                    )


def main() -> None:
    """执行只读审计，并在任一规范被破坏时返回失败。"""
    errors: list[str] = []
    audit_workspace(errors)
    source_files = [
        path
        for path in sorted((ROOT / "crates").glob("**/*.rs"))
        if "/tests/ui/" not in path.as_posix()
    ]

    for path in source_files:
        relative = path.relative_to(ROOT)
        if path.name == "mod.rs":
            errors.append(f"禁止 legacy mod.rs：{relative}")
        for component in relative.parts[1:-1]:
            if not SNAKE_COMPONENT.fullmatch(component):
                errors.append(f"目录不是 snake_case：{relative}")
                break
        if path.name not in {"lib.rs", "main.rs"}:
            stem = path.stem
            if not SNAKE_COMPONENT.fullmatch(stem):
                errors.append(f"文件名不是 snake_case：{relative}")

        lines = path.read_text(encoding="utf-8").splitlines()
        if not any(line.startswith("//!") and HAN.search(line) for line in lines[:24]):
            errors.append(f"模块缺少中文 rustdoc：{relative}")
        if GLOB_REEXPORT.search("\n".join(lines)):
            errors.append(f"禁止 glob 公开重导出：{relative}")

        for index, line in enumerate(lines):
            match = TYPE_DECLARATION.match(line)
            if match and not PASCAL_CASE.fullmatch(match["name"]):
                errors.append(
                    f"类型名不是 PascalCase：{relative}:{index + 1}:{match['name']}"
                )
            declaration = (
                match
                or FUNCTION_DECLARATION.match(line)
                or PUBLIC_FIELD.match(line)
                or PUBLIC_MODULE.match(line)
                or CONSTANT_DECLARATION.match(line)
            )
            if declaration and not has_nearby_chinese_doc(lines, index):
                errors.append(f"声明缺少中文 rustdoc：{relative}:{index + 1}")

    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        raise SystemExit(1)

    print(
        f"Rust 规范审计通过：虚拟 Workspace 与 8 个成员配置一致；"
        f"{len(source_files)} 个源码文件，"
        "目录/文件 snake_case，类型 PascalCase，无 mod.rs、无 glob 重导出，"
        "模块、对象、字段和方法均含中文 rustdoc"
    )


if __name__ == "__main__":
    main()
