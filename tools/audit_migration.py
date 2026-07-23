#!/usr/bin/env python3
"""Audit the one-to-one Java-to-Rust migration contract."""

from __future__ import annotations

import argparse
import csv
import re
from collections import Counter
from pathlib import Path


EXPECTED_FILES = 410
EXPECTED_TEST_SCENARIOS = 332
SNAKE_CASE = re.compile(r"^[a-z][a-z0-9_]*\.rs$")
PLACEHOLDER = re.compile(r"\b(?:placeholder|stub|TODO|unimplemented!|todo!)\b", re.I)
PROHIBITED_CALL = re.compile(r"(?:\bpanic!\s*\(|\.(?:unwrap|expect)\s*\()")
# Match generate_migration_inventory.py: never treat build/index trees as migration sources.
IGNORED_PATH_PARTS = {".git", "target", ".codegraph"}
REQUIRED_COLUMNS = {
    "java_path",
    "rust_path",
    "kind",
    "java_type",
    "rust_type",
    "scenario_count",
    "implementation_status",
    "test_status",
    "wire_fixture",
}


def duplicates(values: list[str]) -> list[str]:
    return sorted(value for value, count in Counter(values).items() if count > 1)


def iter_rust_sources(root: Path) -> list[Path]:
    """Yield workspace `.rs` files, skipping Cargo/trybuild `target` and index dirs."""
    return [
        path
        for path in (root / "crates").rglob("*.rs")
        if IGNORED_PATH_PARTS.isdisjoint(path.relative_to(root).parts)
    ]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--mapping", type=Path, default=Path("docs/migration/file_mapping.csv")
    )
    parser.add_argument("--allow-missing", action="store_true")
    parser.add_argument(
        "--infrastructure",
        type=Path,
        default=Path("docs/migration/infrastructure_files.txt"),
    )
    args = parser.parse_args()

    root = Path(__file__).resolve().parents[1]
    with (root / args.mapping).open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        rows = list(reader)

    errors: list[str] = []
    rust_source_paths = iter_rust_sources(root)
    all_rust_sources = {path.relative_to(root).as_posix() for path in rust_source_paths}
    infrastructure = {
        line.strip()
        for line in (root / args.infrastructure).read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    }
    mapped_paths = {row["rust_path"] for row in rows}
    unregistered = sorted(all_rust_sources - mapped_paths - infrastructure)
    stale_infrastructure = sorted(infrastructure - all_rust_sources)
    if unregistered:
        errors.append(f"unregistered Rust migration files: {', '.join(unregistered)}")
    if stale_infrastructure:
        errors.append(f"stale infrastructure file registrations: {', '.join(stale_infrastructure)}")
    rust_source_text = {
        path.relative_to(root).as_posix(): path.read_text(encoding="utf-8")
        for path in rust_source_paths
    }
    for path, source in rust_source_text.items():
        if "/tests/ui/fail_" not in path and PROHIBITED_CALL.search(source):
            errors.append(f"unexplained panic/unwrap/expect in Rust source: {path}")
    missing_columns = REQUIRED_COLUMNS.difference(reader.fieldnames or [])
    if missing_columns:
        errors.append(f"missing mapping columns: {', '.join(sorted(missing_columns))}")
    if len(rows) != EXPECTED_FILES:
        errors.append(f"expected {EXPECTED_FILES} mappings, found {len(rows)}")

    scenario_count = sum(int(row.get("scenario_count") or 0) for row in rows)
    if scenario_count < EXPECTED_TEST_SCENARIOS:
        errors.append(
            f"expected at least {EXPECTED_TEST_SCENARIOS} Java test scenarios, found {scenario_count}"
        )

    for column in ("java_path", "rust_path"):
        repeated = duplicates([row[column] for row in rows])
        if repeated:
            errors.append(f"duplicate {column}: {', '.join(repeated)}")

    missing: list[str] = []
    incomplete: list[str] = []
    for row in rows:
        rust_path = Path(row["rust_path"])
        if any("-" in part for part in rust_path.parts):
            errors.append(f"non-snake_case path component: {rust_path}")
        if not SNAKE_CASE.fullmatch(rust_path.name):
            errors.append(f"non-snake_case Rust filename: {rust_path}")
        target = root / rust_path
        if not target.is_file():
            missing.append(str(rust_path))
            continue
        source = target.read_text(encoding="utf-8")
        if not source.strip():
            errors.append(f"empty mapped Rust file: {rust_path}")
        if PLACEHOLDER.search(source):
            errors.append(f"placeholder marker in mapped Rust file: {rust_path}")
        # Java test classes map to Cargo-discovered Rust test modules/functions;
        # they do not need an artificial same-named wrapper type. Production and
        # generated/template mappings must still expose their PascalCase type.
        if (
            row.get("kind") not in {"test", "golden"}
            and row.get("java_type")
            and not row.get("rust_type")
        ):
            errors.append(f"mapped Rust file exposes no PascalCase type: {rust_path}")
        module_name = rust_path.stem
        referenced = any(
            path != rust_path.as_posix()
            and (
                re.search(rf"\bmod\s+{re.escape(module_name)}\s*;", text)
                or rust_path.name in text
            )
            for path, text in rust_source_text.items()
        )
        if not referenced:
            errors.append(f"mapped Rust file is not declared or Cargo-discovered: {rust_path}")
        if row.get("implementation_status") != "implemented":
            incomplete.append(str(rust_path))
            if not args.allow_missing:
                errors.append(f"existing file not marked implemented: {rust_path}")
        if row.get("kind") in {"test", "golden"} and row.get("test_status") != "tested":
            if str(rust_path) not in incomplete:
                incomplete.append(str(rust_path))
            if not args.allow_missing:
                errors.append(f"mapped test not marked tested: {rust_path}")

    if missing and not args.allow_missing:
        errors.append(f"missing {len(missing)} mapped Rust files")

    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        if missing:
            for path in missing[:20]:
                print(f"MISSING: {path}")
            if len(missing) > 20:
                print(f"MISSING: ... and {len(missing) - 20} more")
        raise SystemExit(1)

    print(
        f"migration mapping valid: {len(rows)} unique Java files -> "
        f"{len(rows) - len(missing) - len(incomplete)} completed Rust files, "
        f"{len(missing) + len(incomplete)} pending; "
        f"{scenario_count} Java test scenarios"
    )


if __name__ == "__main__":
    main()
