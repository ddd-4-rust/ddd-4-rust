#!/usr/bin/env python3
"""Generate Rust-token templates, golden outputs, and mapped codegen parity tests."""

from __future__ import annotations

import csv
from pathlib import Path

from add_chinese_rustdoc import document_file

ROOT = Path(__file__).resolve().parents[1]
ROWS = list(csv.DictReader((ROOT / "docs/migration/file_mapping.csv").open()))


def rust_ident(java_type: str, stem: str) -> str:
    primary = java_type.split("|")[0]
    if primary == "$" or not primary:
        return "Generated" + "".join(part.title() for part in stem.split("_"))
    return primary.replace("VO", "Vo")


def token_source(type_name: str, derive: str, field_type: str, java_path: str) -> str:
    return (
        f"//! Rust token golden translated from `{java_path}`.\n\n"
        f"use ddd_4_rust_codegen_processor::{derive};\n\n"
        f"#[derive(Debug, Clone, PartialEq, Eq, {derive})]\n"
        f"pub struct {type_name}(pub {field_type});\n"
    )


for row in ROWS:
    if not row["java_path"].startswith("codegen/") or row["kind"] not in {"template", "golden"}:
        continue
    target = ROOT / row["rust_path"]
    target.parent.mkdir(parents=True, exist_ok=True)
    stem = target.stem
    type_name = rust_ident(row["java_type"], stem)
    lower = row["java_path"].lower()
    if "event" in lower:
        source = token_source(type_name, "EventValueObject", "String", row["java_path"])
    elif "rampid" in lower or "integerentityid" in lower:
        source = token_source(type_name, "IntegerEntityIdValueObject", "i32", row["java_path"])
    elif "companykey" in lower or "stringvo" in lower:
        source = token_source(type_name, "StringValueObject", "String", row["java_path"])
    else:
        source = token_source(type_name, "AggregateRootUuidValueObject", "uuid::Uuid", row["java_path"])
    target.write_text(source, encoding="utf-8")
    document_file(target)

test_groups = {
    "aggregate_root_uuid_vo_template_test.rs": "company_id",
    "event_vo_template_test.rs": "event",
    "integer_entity_id_vo_template_test.rs": "ramp_id",
    "string_vo_template_test.rs": "company_key",
}
tests = [row for row in ROWS if row["java_path"].startswith("codegen/processor/src/test/java/")]
for row in tests:
    target = ROOT / row["rust_path"]
    target.parent.mkdir(parents=True, exist_ok=True)
    count = int(row["scenario_count"])
    if count:
        group = test_groups[target.name]
        body = [f"//! Java source: `{row['java_path']}`.\n", "use super::common::assert_golden;\n"]
        for index in range(1, count + 1):
            body.append(f"#[test]\nfn java_scenario_{index}() -> Result<(), syn::Error> {{ assert_golden(\"{group}\", {index}) }}\n")
        source = "\n".join(body)
    else:
        type_name = rust_ident(row["java_type"], target.stem)
        source = f"//! Java source: `{row['java_path']}`.\n\n#[derive(Debug, Clone, Copy, Default)]\npub struct {type_name};\n"
    target.write_text(source, encoding="utf-8")
    document_file(target)

harness = ["//! Cargo-discovered processor template parity tests.\n", "#![expect(dead_code, reason = \"mapped Java test helpers are compile-time fixtures\")]\n", "#[path = \"java_parity/common.rs\"]\nmod common;\n"]
for row in tests:
    relative = Path(row["rust_path"]).relative_to("crates/codegen/processor/tests").as_posix()
    harness.append(f"#[path = \"{relative}\"]\nmod {Path(row['rust_path']).stem};\n")
harness_target = ROOT / "crates/codegen/processor/tests/java_parity.rs"
harness_target.write_text("\n".join(harness), encoding="utf-8")
document_file(harness_target)
