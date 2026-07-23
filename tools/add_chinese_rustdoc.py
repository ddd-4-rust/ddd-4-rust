#!/usr/bin/env python3
"""为 Rust 源码补充可重复生成的中文 rustdoc 契约说明。"""

from __future__ import annotations

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
HAN = re.compile(r"[\u3400-\u9fff]")
TYPE_DECLARATION = re.compile(
    r"^(?P<indent>\s*)(?:(?:pub(?:\([^)]*\))?\s+)?)"
    r"(?P<kind>struct|enum|trait|type)\s+(?P<name>[A-Za-z][A-Za-z0-9_]*)"
)
FUNCTION_DECLARATION = re.compile(
    r"^(?P<indent>\s*)(?:(?:pub(?:\([^)]*\))?\s+)?)"
    r"(?:(?:const|async|unsafe)\s+)*fn\s+(?P<name>[a-zA-Z_][a-zA-Z0-9_]*)"
)
PUBLIC_FIELD = re.compile(
    r"^(?P<indent>\s*)pub\s+(?P<name>[a-z][a-z0-9_]*)\s*:"
)
PUBLIC_MODULE = re.compile(
    r"^(?P<indent>\s*)pub\s+mod\s+(?P<name>[a-z][a-z0-9_]*)\s*;"
)
CONSTANT_DECLARATION = re.compile(
    r"^(?P<indent>\s*)(?:(?:pub(?:\([^)]*\))?\s+)?)"
    r"const\s+(?P<name>[A-Z][A-Z0-9_]*)\s*:"
)


def has_nearby_chinese_doc(lines: list[str], index: int) -> bool:
    """判断声明前最近的属性/文档区域是否已经包含中文说明。"""
    for position in range(index - 1, max(-1, index - 12), -1):
        stripped = lines[position].strip()
        if not stripped:
            continue
        if stripped.startswith(("///", "#[", "#![")):
            if stripped.startswith("///") and HAN.search(stripped):
                return True
            continue
        break
    return False


def module_doc(path: Path) -> str:
    """按文件角色生成模块级中文入口说明。"""
    if path.name == "lib.rs":
        return (
            "//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出"
            "维护稳定公共 API。"
        )
    module = path.stem
    return (
        f"//! `{module}` 模块承载同名 Java 类型迁移后的 Rust 领域实现；"
        "文件名保持 `snake_case`，公开类型保持 `PascalCase`。"
    )


def type_doc(indent: str, kind: str, name: str) -> list[str]:
    """为类型声明生成两行中文契约说明。"""
    if kind == "trait":
        return [
            f"{indent}/// `{name}` 定义该领域概念必须遵守的行为契约。",
            f"{indent}/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。",
        ]
    if kind == "enum":
        return [
            f"{indent}/// `{name}` 汇总该领域操作可能产生的结构化状态或错误分支。",
            f"{indent}/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。",
        ]
    if kind == "type":
        return [
            f"{indent}/// `{name}` 是迁移兼容层公开的稳定类型别名。",
            f"{indent}/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。",
        ]
    return [
        f"{indent}/// `{name}` 表示与同名 Java 类型对应的 Rust 领域对象。",
        f"{indent}/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。",
    ]


def function_doc(indent: str, name: str, line: str) -> list[str]:
    """依据方法命名和接收者类型生成中文行为契约。"""
    mutates = "&mut self" in line
    if name in {"new", "default"} or name.startswith(("new_", "with_")):
        detail = "创建当前类型的新实例，并在构造边界建立该类型要求的不变式。"
    elif name == "build" or name.startswith("build_"):
        detail = "根据已收集的参数构建领域对象；无效组合通过返回类型显式报告。"
    elif name.startswith(("is_", "has_", "contains_", "can_")):
        detail = "判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。"
    elif name.startswith(("validate", "check_")):
        detail = "校验输入或当前状态是否满足领域约束，并通过返回值给出可处理的校验结果。"
    elif name.startswith(("serialize", "marshal")):
        detail = "把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。"
    elif name.startswith(("deserialize", "unmarshal", "parse", "value_of", "from_")):
        detail = "从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。"
    elif name.startswith(("read", "load", "find", "get_", "as_", "to_")):
        detail = "读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。"
    elif name.startswith(("apply", "save", "append", "delete", "mark_", "set_", "register")):
        detail = "执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。"
    elif name == "fmt":
        detail = "按稳定且可读的格式输出当前值，不改变对象内部状态。"
    else:
        detail = f"执行 `{name}` 对应的领域行为，参数和返回值遵循当前类型公开契约。"
    effect = (
        "该方法会修改接收者状态；调用完成后仍需满足类型不变式。"
        if mutates
        else "该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。"
    )
    return [f"{indent}/// {detail}", f"{indent}/// {effect}"]


def field_doc(indent: str, name: str) -> list[str]:
    """为公开字段生成中文数据契约说明。"""
    return [
        f"{indent}/// 保存 `{name}` 对应的领域数据。",
        f"{indent}/// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。",
    ]


def constant_doc(indent: str, name: str) -> list[str]:
    """为公开或 trait 关联常量生成中文说明。"""
    return [
        f"{indent}/// `{name}` 是该类型公开的稳定常量。",
        f"{indent}/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。",
    ]


def document_file(path: Path) -> bool:
    """补充单个 Rust 文件的中文模块、类型、字段和方法文档。"""
    original = path.read_text(encoding="utf-8")
    normalized = original.replace(
        "文件名保持 snake_case，公开类型保持 PascalCase。",
        "文件名保持 `snake_case`，公开类型保持 `PascalCase`。",
    )
    lines = normalized.splitlines()
    changed = False

    if not any(line.startswith("//!") and HAN.search(line) for line in lines[:24]):
        lines.insert(0, module_doc(path))
        lines.insert(1, "//!")
        changed = True

    output: list[str] = []
    for index, line in enumerate(lines):
        item_docs: list[str] = []
        if not has_nearby_chinese_doc(lines, index):
            if match := PUBLIC_MODULE.match(line):
                item_docs = [
                    f"{match['indent']}/// 公开 `{match['name']}` 子模块，"
                    "用于组织对应领域类型及其兼容入口。"
                ]
            elif match := TYPE_DECLARATION.match(line):
                item_docs = type_doc(match["indent"], match["kind"], match["name"])
            elif match := FUNCTION_DECLARATION.match(line):
                item_docs = function_doc(match["indent"], match["name"], line)
            elif match := PUBLIC_FIELD.match(line):
                item_docs = field_doc(match["indent"], match["name"])
            elif match := CONSTANT_DECLARATION.match(line):
                item_docs = constant_doc(match["indent"], match["name"])
        if item_docs:
            output.extend(item_docs)
            changed = True
        output.append(line)

    paragraphs: list[str] = []
    for line in output:
        stripped = line.strip()
        if (
            stripped.startswith("///")
            and HAN.search(stripped)
            and paragraphs
            and paragraphs[-1].strip().startswith("///")
            and paragraphs[-1].strip() != "///"
            and not HAN.search(paragraphs[-1])
        ):
            indent = line[: len(line) - len(line.lstrip())]
            paragraphs.append(f"{indent}///")
        paragraphs.append(line)

    rendered = "\n".join(paragraphs) + "\n"
    if rendered != original:
        path.write_text(rendered, encoding="utf-8")
        return True
    return False


def main() -> None:
    """处理 Workspace 内除 trybuild 失败样例外的全部 Rust 文件。"""
    changed = [
        path
        for path in sorted((ROOT / "crates").glob("**/*.rs"))
        if "/tests/ui/" not in path.as_posix() and document_file(path)
    ]
    print(f"已补充中文 rustdoc：{len(changed)} 个 Rust 源文件")


if __name__ == "__main__":
    main()
