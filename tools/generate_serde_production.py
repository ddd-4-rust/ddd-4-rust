#!/usr/bin/env python3
"""Generate the mechanical namespace wrappers for the three frozen Java serializer APIs."""

from __future__ import annotations

import csv
from pathlib import Path

from add_chinese_rustdoc import document_file

ROOT = Path(__file__).resolve().parents[1]
ROWS = list(csv.DictReader((ROOT / "docs/migration/file_mapping.csv").open()))

ALIASES = {
    "AbstractAggregateExceptionData": "AbstractAggregateExceptionData",
    "AbstractDomainEvent": "CompatAbstractDomainEvent",
    "AbstractEvent": "CompatAbstractEvent",
    "AbstractVersionedAggregateExceptionData": "AbstractVersionedAggregateExceptionData",
    "AggregateAlreadyExistsExceptionData": "AggregateAlreadyExistsExceptionData",
    "AggregateDeletedExceptionData": "AggregateDeletedExceptionData",
    "AggregateNotFoundExceptionData": "AggregateNotFoundExceptionData",
    "AggregateVersionConflictExceptionData": "AggregateVersionConflictExceptionData",
    "AggregateVersionNotFoundExceptionData": "AggregateVersionNotFoundExceptionData",
    "DecryptionFailedExceptionData": "DecryptionFailedExceptionData",
    "DuplicateEncryptionKeyIdExceptionData": "DuplicateEncryptionKeyIdExceptionData",
    "DuplicateEntityExceptionData": "DuplicateEntityExceptionData",
    "EncryptedDataJackson": "CompatEncryptedData",
    "EncryptedDataJsonb": "CompatEncryptedData",
    "EncryptedDataJaxb": "CompatEncryptedData",
    "EncryptionKeyIdUnknownExceptionData": "EncryptionKeyIdUnknownExceptionData",
    "EncryptionKeyVersionUnknownExceptionData": "EncryptionKeyVersionUnknownExceptionData",
    "EntityNotFoundExceptionData": "EntityNotFoundExceptionData",
}


def source_for(domain: str, type_name: str, java_path: str) -> str:
    header = f"//! Java source: `{java_path}`.\n\n"
    primary = type_name.split("|")[0]
    if domain == "jackson" and primary in ALIASES:
        serde_type = "EncryptedData" if primary == "EncryptedDataJackson" else primary
        return header + f"/// Legacy alias backed by the Serde-native API.\npub type {primary} = crate::json::serde::{serde_type};\n"
    if primary in ALIASES:
        return header + f"/// Compatibility type backed by the private shared representation.\npub type {primary} = crate::compat::{ALIASES[primary]};\n"
    if primary in {"AggregateVersionJacksonSerializer"}:
        return header + """/// Legacy name delegating to the Serde-native aggregate-version adapter.
pub struct AggregateVersionJacksonSerializer;
impl AggregateVersionJacksonSerializer {
    #[must_use]
    pub fn serialize(value: &ddd_4_rust_core::AggregateVersion) -> u32 {
        crate::json::serde::AggregateVersionAdapter::serialize(value)
    }
}
"""
    if primary in {"AggregateVersionJacksonDeserializer"}:
        return header + """/// Legacy name delegating to the Serde-native aggregate-version adapter.
pub struct AggregateVersionJacksonDeserializer;
impl AggregateVersionJacksonDeserializer {
    #[must_use]
    pub fn deserialize(value: u32) -> ddd_4_rust_core::AggregateVersion {
        crate::json::serde::AggregateVersionAdapter::deserialize(value)
    }
}
"""
    if primary in {"AggregateVersionJsonbAdapter", "AggregateVersionXmlAdapter"}:
        return header + f"""/// {domain.upper()} aggregate-version adapter.
pub struct {primary};
impl {primary} {{
    #[must_use]
    pub fn marshal(value: Option<&ddd_4_rust_core::AggregateVersion>) -> Option<u32> {{
        value.map(ddd_4_rust_core::AggregateVersion::as_u32)
    }}

    #[must_use]
    pub fn unmarshal(value: Option<u32>) -> Option<ddd_4_rust_core::AggregateVersion> {{
        value.map(ddd_4_rust_core::AggregateVersion::new)
    }}
}}
"""
    if primary in {"EntityIdJacksonSerializer"}:
        return header + """/// Legacy name delegating to the Serde-native entity-ID adapter.
pub struct EntityIdJacksonSerializer;
impl EntityIdJacksonSerializer {
    #[must_use]
    pub fn serialize(value: &dyn ddd_4_rust_core::EntityId) -> String {
        crate::json::serde::EntityIdAdapter::serialize(value)
    }
}
"""
    if primary in {"EntityIdJacksonDeserializer"}:
        return header + """/// Legacy name delegating to the Serde-native entity-ID adapter.
pub struct EntityIdJacksonDeserializer;
impl EntityIdJacksonDeserializer {
    #[must_use]
    pub fn deserialize(
        factory: &dyn ddd_4_rust_core::EntityIdFactory,
        value: Option<&str>,
    ) -> Option<Box<dyn ddd_4_rust_core::EntityId>> {
        crate::json::serde::EntityIdAdapter::deserialize(factory, value)
    }
}
"""
    if primary in {"EntityIdJsonbAdapter", "EntityIdXmlAdapter"}:
        return header + f"""/// {domain.upper()} typed entity-ID adapter.
pub struct {primary};
impl {primary} {{
    #[must_use]
    pub fn marshal(value: Option<&dyn ddd_4_rust_core::EntityId>) -> Option<String> {{
        value.map(ddd_4_rust_core::EntityId::as_typed_string)
    }}

    #[must_use]
    pub fn unmarshal(
        factory: &dyn ddd_4_rust_core::EntityIdFactory,
        value: Option<&str>,
    ) -> Option<Box<dyn ddd_4_rust_core::EntityId>> {{
        value
            .and_then(|value| value.find(' ').map(|position| (&value[..position], &value[position + 1..])))
            .and_then(|(kind, id)| factory.create_entity_id(kind, id))
    }}
}}
"""
    if primary == "EntityIdPathJacksonDeserializer":
        return header + """/// Legacy name delegating to the Serde-native entity-ID-path adapter.
pub struct EntityIdPathJacksonDeserializer;
impl EntityIdPathJacksonDeserializer {
    #[must_use]
    pub fn marshal(value: Option<&ddd_4_rust_core::EntityIdPath>) -> Option<String> {
        crate::json::serde::EntityIdPathAdapter::serialize(value)
    }

    #[must_use]
    pub fn unmarshal(
        factory: &dyn ddd_4_rust_core::EntityIdFactory,
        value: Option<&str>,
    ) -> Option<ddd_4_rust_core::EntityIdPath> {
        crate::json::serde::EntityIdPathAdapter::deserialize(factory, value)
    }
}
"""
    if primary in {"EntityIdPathJsonbAdapter", "EntityIdPathXmlAdapter"}:
        return header + f"""/// {domain.upper()} entity-ID-path adapter.
pub struct {primary};
impl {primary} {{
    #[must_use]
    pub fn marshal(value: Option<&ddd_4_rust_core::EntityIdPath>) -> Option<String> {{
        value.map(ddd_4_rust_core::EntityIdPath::as_base_type)
    }}

    #[must_use]
    pub fn unmarshal(
        factory: &dyn ddd_4_rust_core::EntityIdFactory,
        value: Option<&str>,
    ) -> Option<ddd_4_rust_core::EntityIdPath> {{
        ddd_4_rust_core::EntityIdPath::value_of(factory, value)
    }}
}}
"""
    if primary in {"EventIdJsonbAdapter", "EventIdXmlAdapter"}:
        return header + f"""/// {domain.upper()} event-ID adapter.
pub struct {primary};
impl {primary} {{
    #[must_use]
    pub fn marshal(value: Option<&ddd_4_rust_core::EventId>) -> Option<String> {{
        value.map(ddd_4_rust_core::EventId::as_string)
    }}

    #[must_use]
    pub fn unmarshal(value: Option<&str>) -> Option<ddd_4_rust_core::EventId> {{
        value.and_then(ddd_4_rust_core::EventId::value_of)
    }}
}}
"""
    if primary == "Ddd4JacksonModule":
        return header + "/// Legacy alias for the Serde-native module marker.\npub type Ddd4JacksonModule = crate::DddSerdeModule;\n"
    raise ValueError(f"no production generator for {primary}")


def generate_domain(domain: str) -> None:
    rows = [row for row in ROWS if row["java_path"].startswith(f"{domain}/") and row["kind"] == "production"]
    modules: list[tuple[str, str]] = []
    for row in rows:
        target = ROOT / row["rust_path"]
        target.parent.mkdir(parents=True, exist_ok=True)
        primary = row["java_type"].split("|")[0]
        target.write_text(source_for(domain, row["java_type"], row["java_path"]), encoding="utf-8")
        document_file(target)
        modules.append((target.stem, primary))
    namespace = ROOT / (f"crates/serde/src/json/{domain}.rs" if domain in {"jackson", "jsonb"} else "crates/serde/src/xml/jaxb.rs")
    namespace.parent.mkdir(parents=True, exist_ok=True)
    if domain == "jackson":
        body = ["//! Deprecated Java/Jackson naming facade backed entirely by `json::serde`.\n"]
    else:
        body = [f"//! Java-compatible {domain.upper()} namespace.\n"]
    for module, primary in modules:
        body.append(f"mod {module};\npub use {module}::{primary};\n")
    namespace.write_text("\n".join(body), encoding="utf-8")
    document_file(namespace)


for selected in ("jackson", "jsonb", "jaxb"):
    generate_domain(selected)
