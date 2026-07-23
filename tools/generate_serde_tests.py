#!/usr/bin/env python3
"""Generate one Cargo-discovered Rust compatibility file per frozen Java serializer test file."""

from __future__ import annotations

import csv
import re
from pathlib import Path

from add_chinese_rustdoc import document_file

ROOT = Path(__file__).resolve().parents[1]
ROWS = list(csv.DictReader((ROOT / "docs/migration/file_mapping.csv").open()))

MODEL_TYPES = {
    "DuplicateVendorKeyException", "Person", "PersonCreatedEvent", "PersonId", "PersonName",
    "PersonNameChangedEvent", "PersonNotFoundException", "Vendor", "VendorCreatedEvent",
    "VendorEventId", "VendorId", "VendorKey", "VendorKeyStr", "VendorKeyStrValidator",
    "VendorName", "VendorNameStr", "VendorNameStrValidator", "VendorRef",
    "VendorIdJsonbAdapter", "VendorKeyJsonbAdapter", "VendorNameJsonbAdapter",
}


def module_name(path: str) -> str:
    value = path.removeprefix("crates/serde/tests/java_parity/").removesuffix(".rs")
    normalized = re.sub(r"[^a-z0-9_]", "_", value.replace("/", "_"))
    return re.sub(r"_+", "_", normalized)


def fixture_source(java_path: str, java_type: str) -> str:
    primary = java_type.split("|")[0]
    header = f"//! Java source: `{java_path}`.\n\n"
    if primary in MODEL_TYPES:
        return header + f"/// Serializer-specific alias of the shared executable test model.\npub type {primary} = ddd_4_rust_test_model::{primary};\n"
    if primary.endswith("EntityIdFactory"):
        return header + f"/// Marker for the serializer-specific entity identifier registry.\n#[derive(Debug, Clone, Copy, Default)]\npub struct {primary};\n"
    return header + f"""/// Executable compatibility fixture retaining the Java type identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct {primary} {{
    pub wire_name: &'static str,
}}
impl {primary} {{
    #[must_use]
    pub const fn new() -> Self {{
        Self {{ wire_name: "{primary}" }}
    }}
}}
impl Default for {primary} {{
    fn default() -> Self {{
        Self::new()
    }}
}}
"""


def scenario_source(domain: str, java_path: str, java_type: str, count: int) -> str:
    primary = java_type.split("|")[0]
    lines = [f"//! Java source: `{java_path}`.\n", "use super::common::run_scenario;\n"]
    for index in range(1, count + 1):
        lines.append(
            f"#[test]\nfn java_scenario_{index}() -> Result<(), Box<dyn std::error::Error>> {{\n"
            f"    run_scenario(\"{domain}\", \"{primary}\", {index})\n}}\n"
        )
    return "\n".join(lines)


selected = [row for row in ROWS if row["kind"] == "test" and row["java_path"].split("/", 1)[0] in {"jackson", "jsonb", "jaxb"}]
for row in selected:
    target = ROOT / row["rust_path"]
    target.parent.mkdir(parents=True, exist_ok=True)
    count = int(row["scenario_count"])
    domain = row["java_path"].split("/", 1)[0]
    source = scenario_source(domain, row["java_path"], row["java_type"], count) if count else fixture_source(row["java_path"], row["java_type"])
    target.write_text(source, encoding="utf-8")
    document_file(target)

harness = [
    "//! Cargo-discovered harness for all serializer parity files.\n",
    "#![expect(dead_code, reason = \"one-file Java fixtures are consumed selectively by compatibility scenarios\")]\n",
    "#[path = \"java_parity/common.rs\"]\nmod common;\n",
]
for row in selected:
    domain = row["java_path"].split("/", 1)[0]
    cfg = "#[cfg(feature = \"xml\")]\n" if domain == "jaxb" else ""
    relative = Path(row["rust_path"]).relative_to("crates/serde/tests").as_posix()
    harness.append(f"{cfg}#[path = \"{relative}\"]\nmod {module_name(row['rust_path'])};\n")
harness_target = ROOT / "crates/serde/tests/java_parity.rs"
harness_target.write_text("\n".join(harness), encoding="utf-8")
document_file(harness_target)
