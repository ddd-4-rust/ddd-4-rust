#!/usr/bin/env python3
"""Generate the frozen ddd-4-java 0.7.0 to Rust file inventory."""

from __future__ import annotations

import argparse
import csv
import re
import subprocess
from pathlib import Path


EXPECTED_FILES = 410
JAVA_BASELINE = "baa9a989"
JAVA_TYPE = re.compile(
    r"(?:public\s+)?(?:abstract\s+|final\s+)?(?:class|interface|enum|record|@interface)\s+([A-Za-z_$][A-Za-z0-9_$]*)"
)
RUST_TYPE = re.compile(
    r"\bpub(?:\([^)]*\))?\s+(?:struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)"
)
JAVA_TEST = re.compile(r"@(Test|ParameterizedTest|RepeatedTest)\b")
RUST_TEST = re.compile(r"#\s*\[\s*(?:[A-Za-z0-9_:]+::)?test\s*\]")
PLACEHOLDER = re.compile(r"\b(?:placeholder|stub|TODO|unimplemented!|todo!)\b", re.I)


def snake_case(name: str) -> str:
    """Convert a Java source basename to a stable Rust module basename."""
    if name == "package-info":
        return "package_info"
    value = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value).lower()
    return value.replace("ddd4_j", "ddd4j")


def java_files(java_root: Path) -> list[str]:
    output = subprocess.check_output(
        ["git", "ls-tree", "-r", "--name-only", JAVA_BASELINE],
        cwd=java_root,
        text=True,
    )
    return sorted(
        path
        for path in output.splitlines()
        if path.endswith(".java")
        and path != ".mvn/wrapper/MavenWrapperDownloader.java"
    )


def java_source(java_root: Path, java_path: str) -> str:
    """Read source from the immutable Java baseline commit."""
    return subprocess.check_output(
        ["git", "show", f"{JAVA_BASELINE}:{java_path}"], cwd=java_root, text=True
    )


def type_names(pattern: re.Pattern[str], source: str) -> str:
    """Return stable pipe-separated declaration names."""
    without_comments = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.S)
    return "|".join(dict.fromkeys(pattern.findall(without_comments)))


def has_implementation(path: Path) -> bool:
    """Reject empty, comments-only, and explicitly unfinished Rust files."""
    if not path.is_file():
        return False
    source = path.read_text(encoding="utf-8")
    without_comments = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.S)
    return bool(without_comments.strip()) and PLACEHOLDER.search(source) is None


def classify(java_path: str) -> str:
    if "/src/main/java/" in java_path:
        return "production"
    if "/src/test/java/" in java_path:
        return "test"
    if "/src-gen/main/java/" in java_path:
        return "generated"
    if "/src/main/resources/" in java_path:
        return "template"
    if "/src/test/resources/" in java_path:
        return "golden"
    raise ValueError(f"unsupported Java source layout: {java_path}")


def package_leaf(java_path: str, marker: str) -> str:
    relative = java_path.split(marker, 1)[1]
    parts = relative.split("/")[:-1]
    return snake_case(parts[-1]) if parts else "root"


def target_path(java_path: str) -> str:
    basename = snake_case(Path(java_path).stem) + ".rs"
    kind = classify(java_path)
    top = java_path.split("/", 1)[0]

    if top in {"core", "esc"}:
        if kind == "production":
            return f"crates/{top}/src/{basename}"
        package = package_leaf(java_path, "/src/test/java/")
        return f"crates/{top}/tests/java_parity/{package}/{basename}"

    if top in {"jackson", "jsonb", "jaxb"}:
        format_name = "xml" if top == "jaxb" else "json"
        if kind == "production":
            return f"crates/serde/src/{format_name}/{top}/{basename}"
        package = package_leaf(java_path, "/src/test/java/")
        return f"crates/serde/tests/java_parity/{top}/{package}/{basename}"

    if top == "jsonb-testmodel":
        if kind == "production":
            return f"crates/test/model/src/{basename}"
        return f"crates/test/model/tests/java_parity/{basename}"

    if top == "junit":
        return f"crates/test/support/src/{basename}"

    if top == "jacoco":
        return "crates/test/support/src/coverage_marker.rs"

    if top == "codegen":
        second = java_path.split("/", 2)[1]
        crate = {
            "api": "crates/codegen/api",
            "processor": "crates/codegen/processor",
            "example": "crates/codegen/example",
        }[second]
        if kind == "production":
            return f"{crate}/src/{basename}"
        if kind == "test":
            return f"{crate}/tests/java_parity/{basename}"
        if kind == "generated":
            return f"{crate}/generated/{basename}"
        if kind == "template":
            return f"{crate}/templates/{basename}"
        relative = java_path.split("/src/test/resources/expected/", 1)[1]
        parent = snake_case(relative.split("/", 1)[0])
        return f"{crate}/tests/golden/{parent}/{basename}"

    raise ValueError(f"unsupported Java module: {java_path}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("java_root", type=Path)
    parser.add_argument(
        "--output", type=Path, default=Path("docs/migration/file_mapping.csv")
    )
    parser.add_argument(
        "--java-tree", type=Path, default=Path("docs/migration/java-tree-full.md")
    )
    parser.add_argument(
        "--rust-tree", type=Path, default=Path("docs/migration/rust-tree-full.md")
    )
    args = parser.parse_args()

    java_root = args.java_root.resolve()
    rust_root = Path(__file__).resolve().parents[1]
    files = java_files(java_root)
    if len(files) != EXPECTED_FILES:
        raise SystemExit(f"expected {EXPECTED_FILES} Java files, found {len(files)}")

    rows = []
    for java_path in files:
        rust_path = target_path(java_path)
        kind = classify(java_path)
        source = java_source(java_root, java_path)
        target = rust_root / rust_path
        implemented = has_implementation(target)
        rust_source = target.read_text(encoding="utf-8") if target.is_file() else ""
        java_scenarios = len(JAVA_TEST.findall(source)) if kind == "test" else 0
        rust_scenarios = len(RUST_TEST.findall(rust_source)) if kind == "test" else 0
        if kind in {"test", "golden"}:
            tested = implemented and rust_scenarios >= java_scenarios
            test_status = "tested" if tested else "pending"
        else:
            test_status = "not_applicable"
        wire_fixture = (
            java_path
            if java_path.split("/", 1)[0] in {"jackson", "jsonb", "jaxb"}
            or kind == "golden"
            else ""
        )
        rows.append(
            {
                "java_path": java_path,
                "rust_path": rust_path,
                "kind": kind,
                "java_type": type_names(JAVA_TYPE, source),
                "rust_type": type_names(RUST_TYPE, rust_source) if implemented else "",
                "scenario_count": java_scenarios,
                "implementation_status": "implemented" if implemented else "pending",
                "test_status": test_status,
                "wire_fixture": wire_fixture,
            }
        )

    output = rust_root / args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=rows[0].keys())
        writer.writeheader()
        writer.writerows(rows)

    java_tree = rust_root / args.java_tree
    java_tree.write_text(
        "# Frozen Java 0.7.0 migration files\n\n"
        f"Commit: `{JAVA_BASELINE}`; mapped files: **{len(files)}**.\n\n"
        + "\n".join(f"- `{path}`" for path in files)
        + "\n",
        encoding="utf-8",
    )

    ignored_parts = {".git", "target", ".codegraph"}
    rust_files = sorted(
        path.relative_to(rust_root).as_posix()
        for path in rust_root.rglob("*.rs")
        if ignored_parts.isdisjoint(path.relative_to(rust_root).parts)
    )
    rust_tree = rust_root / args.rust_tree
    rust_tree.write_text(
        "# Current Rust source tree\n\n"
        f"Rust source files: **{len(rust_files)}**. Infrastructure files are not part of the 410 mapping.\n\n"
        + "\n".join(f"- `{path}`" for path in rust_files)
        + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
