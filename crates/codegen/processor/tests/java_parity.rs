//! `java_parity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Cargo-discovered processor template parity tests.

#![expect(
    dead_code,
    reason = "mapped Java test helpers are compile-time fixtures"
)]

#[path = "java_parity/common.rs"]
mod common;

#[path = "java_parity/aggregate_root_uuid_vo_template_test.rs"]
mod aggregate_root_uuid_vo_template_test;

#[path = "java_parity/event_vo_template_test.rs"]
mod event_vo_template_test;

#[path = "java_parity/expected_file_matcher.rs"]
mod expected_file_matcher;

#[path = "java_parity/integer_entity_id_vo_template_test.rs"]
mod integer_entity_id_vo_template_test;

#[path = "java_parity/my_id.rs"]
mod my_id;

#[path = "java_parity/string_vo_template_test.rs"]
mod string_vo_template_test;

#[path = "java_parity/test_utils.rs"]
mod test_utils;
