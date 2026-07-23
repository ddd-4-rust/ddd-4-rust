//! `java_parity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Test harness for the one-file-per-Java-test compatibility suite.

#[path = "java_parity/core/base_test.rs"]
mod base_test;

#[path = "java_parity/core/abstract_aggregate_root_test.rs"]
mod abstract_aggregate_root_test;
#[path = "java_parity/core/abstract_entity_test.rs"]
mod abstract_entity_test;
#[path = "java_parity/core/aggregate_already_exists_exception_test.rs"]
mod aggregate_already_exists_exception_test;
#[path = "java_parity/core/aggregate_deleted_exception_test.rs"]
mod aggregate_deleted_exception_test;
#[path = "java_parity/core/aggregate_not_found_exception_test.rs"]
mod aggregate_not_found_exception_test;
#[path = "java_parity/core/aggregate_root_uuid_test.rs"]
mod aggregate_root_uuid_test;
#[path = "java_parity/core/aggregate_version_conflict_exception_test.rs"]
mod aggregate_version_conflict_exception_test;
#[path = "java_parity/core/aggregate_version_not_found_exception_test.rs"]
mod aggregate_version_not_found_exception_test;
#[path = "java_parity/core/aggregate_version_test.rs"]
mod aggregate_version_test;
#[path = "java_parity/core/architecture_test.rs"]
mod architecture_test;
#[path = "java_parity/core/ddd4j_utils_test.rs"]
mod ddd4j_utils_test;
#[path = "java_parity/core/decryption_failed_exception_test.rs"]
mod decryption_failed_exception_test;
#[path = "java_parity/core/domain_event_expected_entity_id_path_validator_test.rs"]
mod domain_event_expected_entity_id_path_validator_test;
#[path = "java_parity/core/duplicate_encryption_key_id_exception_test.rs"]
mod duplicate_encryption_key_id_exception_test;
#[path = "java_parity/core/duplicate_entity_exception_test.rs"]
mod duplicate_entity_exception_test;
#[path = "java_parity/core/encryption_key_id_unknown_exception_test.rs"]
mod encryption_key_id_unknown_exception_test;
#[path = "java_parity/core/encryption_key_version_unknown_exception_test.rs"]
mod encryption_key_version_unknown_exception_test;
#[path = "java_parity/core/entity_id_path_test.rs"]
mod entity_id_path_test;
#[path = "java_parity/core/entity_id_test.rs"]
mod entity_id_test;
#[path = "java_parity/core/entity_not_found_exception_test.rs"]
mod entity_not_found_exception_test;
#[path = "java_parity/core/event_id_test.rs"]
mod event_id_test;
#[path = "java_parity/core/event_type_test.rs"]
mod event_type_test;
#[path = "java_parity/core/expected_entity_id_path_validator_test.rs"]
mod expected_entity_id_path_validator_test;
#[path = "java_parity/core/has_entity_type_constant_validator_test.rs"]
mod has_entity_type_constant_validator_test;
#[path = "java_parity/core/integer_entity_id_test.rs"]
mod integer_entity_id_test;
#[path = "java_parity/core/jandex_entity_id_factory_test.rs"]
mod jandex_entity_id_factory_test;
#[path = "java_parity/core/method_executor_test.rs"]
mod method_executor_test;
#[path = "java_parity/core/string_based_entity_type_test.rs"]
mod string_based_entity_type_test;

#[path = "java_parity/coretest/a_created_event.rs"]
mod a_created_event;
#[path = "java_parity/coretest/a_id.rs"]
mod a_id;
#[path = "java_parity/coretest/a_root.rs"]
mod a_root;
#[path = "java_parity/coretest/abstract_domain_event.rs"]
mod abstract_domain_event;
#[path = "java_parity/coretest/abstract_event.rs"]
mod abstract_event;
#[path = "java_parity/coretest/b_added_event.rs"]
mod b_added_event;
#[path = "java_parity/coretest/b_entity.rs"]
mod b_entity;
#[path = "java_parity/coretest/b_id.rs"]
mod b_id;
#[path = "java_parity/coretest/base_root.rs"]
mod base_root;
#[path = "java_parity/coretest/c_added_event.rs"]
mod c_added_event;
#[path = "java_parity/coretest/c_entity.rs"]
mod c_entity;
#[path = "java_parity/coretest/c_event.rs"]
mod c_event;
#[path = "java_parity/coretest/c_id.rs"]
mod c_id;
#[path = "java_parity/coretest/d_event.rs"]
mod d_event;
#[path = "java_parity/coretest/impl_root_id.rs"]
mod impl_root_id;
#[path = "java_parity/coretest/person_id.rs"]
mod person_id;
#[path = "java_parity/coretest/vendor_id.rs"]
mod vendor_id;
