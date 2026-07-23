//! `java_parity` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Cargo-discovered harness for all serializer parity files.

#![expect(
    dead_code,
    reason = "one-file Java fixtures are consumed selectively by compatibility scenarios"
)]

#[path = "java_parity/common.rs"]
mod common;

#[path = "java_parity/jackson/jackson/abstract_domain_event_test.rs"]
mod jackson_jackson_abstract_domain_event_test;

#[path = "java_parity/jackson/jackson/abstract_event_test.rs"]
mod jackson_jackson_abstract_event_test;

#[path = "java_parity/jackson/jackson/aggregate_already_exists_exception_data_test.rs"]
mod jackson_jackson_aggregate_already_exists_exception_data_test;

#[path = "java_parity/jackson/jackson/aggregate_deleted_exception_data_test.rs"]
mod jackson_jackson_aggregate_deleted_exception_data_test;

#[path = "java_parity/jackson/jackson/aggregate_not_found_exception_data_test.rs"]
mod jackson_jackson_aggregate_not_found_exception_data_test;

#[path = "java_parity/jackson/jackson/aggregate_version_conflict_exception_data_test.rs"]
mod jackson_jackson_aggregate_version_conflict_exception_data_test;

#[path = "java_parity/jackson/jackson/aggregate_version_jackson_deserializer_test.rs"]
mod jackson_jackson_aggregate_version_jackson_deserializer_test;

#[path = "java_parity/jackson/jackson/aggregate_version_jackson_serializer_test.rs"]
mod jackson_jackson_aggregate_version_jackson_serializer_test;

#[path = "java_parity/jackson/jackson/aggregate_version_not_found_exception_data_test.rs"]
mod jackson_jackson_aggregate_version_not_found_exception_data_test;

#[path = "java_parity/jackson/jackson/architecture_test.rs"]
mod jackson_jackson_architecture_test;

#[path = "java_parity/jackson/jackson/base_test.rs"]
mod jackson_jackson_base_test;

#[path = "java_parity/jackson/jackson/decryption_failed_exception_data_test.rs"]
mod jackson_jackson_decryption_failed_exception_data_test;

#[path = "java_parity/jackson/jackson/duplicate_encryption_key_id_exception_data_test.rs"]
mod jackson_jackson_duplicate_encryption_key_id_exception_data_test;

#[path = "java_parity/jackson/jackson/duplicate_entity_exception_data_test.rs"]
mod jackson_jackson_duplicate_entity_exception_data_test;

#[path = "java_parity/jackson/jackson/encrypted_data_jackson_test.rs"]
mod jackson_jackson_encrypted_data_jackson_test;

#[path = "java_parity/jackson/jackson/encryption_key_id_unknown_exception_data_test.rs"]
mod jackson_jackson_encryption_key_id_unknown_exception_data_test;

#[path = "java_parity/jackson/jackson/encryption_key_version_unknown_exception_data_test.rs"]
mod jackson_jackson_encryption_key_version_unknown_exception_data_test;

#[path = "java_parity/jackson/jackson/entity_id_jackson_deserializer_test.rs"]
mod jackson_jackson_entity_id_jackson_deserializer_test;

#[path = "java_parity/jackson/jackson/entity_id_jackson_serializer_test.rs"]
mod jackson_jackson_entity_id_jackson_serializer_test;

#[path = "java_parity/jackson/jackson/entity_id_path_jackson_deserializer_test.rs"]
mod jackson_jackson_entity_id_path_jackson_deserializer_test;

#[path = "java_parity/jackson/jackson/entity_id_path_jackson_serializer_test.rs"]
mod jackson_jackson_entity_id_path_jackson_serializer_test;

#[path = "java_parity/jackson/jackson/entity_not_found_exception_data_test.rs"]
mod jackson_jackson_entity_not_found_exception_data_test;

#[path = "java_parity/jackson/jackson/event_id_jackson_deserializer_test.rs"]
mod jackson_jackson_event_id_jackson_deserializer_test;

#[path = "java_parity/jackson/jackson/event_id_jackson_serializer_test.rs"]
mod jackson_jackson_event_id_jackson_serializer_test;

#[path = "java_parity/jackson/jackson/test_utils.rs"]
mod jackson_jackson_test_utils;

#[path = "java_parity/jackson/jacksontest/a_created_event.rs"]
mod jackson_jacksontest_a_created_event;

#[path = "java_parity/jackson/jacksontest/a_id.rs"]
mod jackson_jacksontest_a_id;

#[path = "java_parity/jackson/jacksontest/a_root.rs"]
mod jackson_jacksontest_a_root;

#[path = "java_parity/jackson/jacksontest/b_added_event.rs"]
mod jackson_jacksontest_b_added_event;

#[path = "java_parity/jackson/jacksontest/b_entity.rs"]
mod jackson_jacksontest_b_entity;

#[path = "java_parity/jackson/jacksontest/b_id.rs"]
mod jackson_jacksontest_b_id;

#[path = "java_parity/jackson/jacksontest/base_root.rs"]
mod jackson_jacksontest_base_root;

#[path = "java_parity/jackson/jacksontest/c_added_event.rs"]
mod jackson_jacksontest_c_added_event;

#[path = "java_parity/jackson/jacksontest/c_entity.rs"]
mod jackson_jacksontest_c_entity;

#[path = "java_parity/jackson/jacksontest/c_event.rs"]
mod jackson_jacksontest_c_event;

#[path = "java_parity/jackson/jacksontest/c_id.rs"]
mod jackson_jacksontest_c_id;

#[path = "java_parity/jackson/jacksontest/d_event.rs"]
mod jackson_jacksontest_d_event;

#[path = "java_parity/jackson/jacksontest/duplicate_vendor_key_exception.rs"]
mod jackson_jacksontest_duplicate_vendor_key_exception;

#[path = "java_parity/jackson/jacksontest/impl_root_id.rs"]
mod jackson_jacksontest_impl_root_id;

#[path = "java_parity/jackson/jacksontest/jackson_test_entity_id_factory.rs"]
mod jackson_jacksontest_jackson_test_entity_id_factory;

#[path = "java_parity/jackson/jacksontest/person.rs"]
mod jackson_jacksontest_person;

#[path = "java_parity/jackson/jacksontest/person_created_event.rs"]
mod jackson_jacksontest_person_created_event;

#[path = "java_parity/jackson/jacksontest/person_id.rs"]
mod jackson_jacksontest_person_id;

#[path = "java_parity/jackson/jacksontest/person_name.rs"]
mod jackson_jacksontest_person_name;

#[path = "java_parity/jackson/jacksontest/person_name_changed_event.rs"]
mod jackson_jacksontest_person_name_changed_event;

#[path = "java_parity/jackson/jacksontest/person_not_found_exception.rs"]
mod jackson_jacksontest_person_not_found_exception;

#[path = "java_parity/jackson/jacksontest/test_jackson_adapter_module.rs"]
mod jackson_jacksontest_test_jackson_adapter_module;

#[path = "java_parity/jackson/jacksontest/vendor.rs"]
mod jackson_jacksontest_vendor;

#[path = "java_parity/jackson/jacksontest/vendor_created_event.rs"]
mod jackson_jacksontest_vendor_created_event;

#[path = "java_parity/jackson/jacksontest/vendor_event_id.rs"]
mod jackson_jacksontest_vendor_event_id;

#[path = "java_parity/jackson/jacksontest/vendor_example_test.rs"]
mod jackson_jacksontest_vendor_example_test;

#[path = "java_parity/jackson/jacksontest/vendor_id.rs"]
mod jackson_jacksontest_vendor_id;

#[path = "java_parity/jackson/jacksontest/vendor_key.rs"]
mod jackson_jacksontest_vendor_key;

#[path = "java_parity/jackson/jacksontest/vendor_key_str.rs"]
mod jackson_jacksontest_vendor_key_str;

#[path = "java_parity/jackson/jacksontest/vendor_key_str_validator.rs"]
mod jackson_jacksontest_vendor_key_str_validator;

#[path = "java_parity/jackson/jacksontest/vendor_name.rs"]
mod jackson_jacksontest_vendor_name;

#[path = "java_parity/jackson/jacksontest/vendor_name_str.rs"]
mod jackson_jacksontest_vendor_name_str;

#[path = "java_parity/jackson/jacksontest/vendor_name_str_validator.rs"]
mod jackson_jacksontest_vendor_name_str_validator;

#[path = "java_parity/jackson/jacksontest/vendor_ref.rs"]
mod jackson_jacksontest_vendor_ref;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/abstract_domain_event_test.rs"]
mod jaxb_jaxb_abstract_domain_event_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/abstract_event_test.rs"]
mod jaxb_jaxb_abstract_event_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_already_exists_exception_data_test.rs"]
mod jaxb_jaxb_aggregate_already_exists_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_deleted_exception_data_test.rs"]
mod jaxb_jaxb_aggregate_deleted_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_not_found_exception_data_test.rs"]
mod jaxb_jaxb_aggregate_not_found_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_version_conflict_exception_data_test.rs"]
mod jaxb_jaxb_aggregate_version_conflict_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_version_not_found_exception_data_test.rs"]
mod jaxb_jaxb_aggregate_version_not_found_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/aggregate_version_xml_adapter_test.rs"]
mod jaxb_jaxb_aggregate_version_xml_adapter_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/architecture_test.rs"]
mod jaxb_jaxb_architecture_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/base_test.rs"]
mod jaxb_jaxb_base_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/decryption_failed_exception_data_test.rs"]
mod jaxb_jaxb_decryption_failed_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/duplicate_encryption_key_id_exception_data_test.rs"]
mod jaxb_jaxb_duplicate_encryption_key_id_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/duplicate_entity_exception_data_test.rs"]
mod jaxb_jaxb_duplicate_entity_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/encrypted_data_jaxb_test.rs"]
mod jaxb_jaxb_encrypted_data_jaxb_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/encryption_key_id_unknown_exception_data_test.rs"]
mod jaxb_jaxb_encryption_key_id_unknown_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/encryption_key_version_unknown_exception_data_test.rs"]
mod jaxb_jaxb_encryption_key_version_unknown_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/entity_id_path_xml_adapter_test.rs"]
mod jaxb_jaxb_entity_id_path_xml_adapter_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/entity_id_xml_adapter_test.rs"]
mod jaxb_jaxb_entity_id_xml_adapter_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/entity_not_found_exception_data_test.rs"]
mod jaxb_jaxb_entity_not_found_exception_data_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxb/event_id_xml_adapter_test.rs"]
mod jaxb_jaxb_event_id_xml_adapter_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/a_created_event.rs"]
mod jaxb_jaxbtest_a_created_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/a_id.rs"]
mod jaxb_jaxbtest_a_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/a_root.rs"]
mod jaxb_jaxbtest_a_root;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/b_added_event.rs"]
mod jaxb_jaxbtest_b_added_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/b_entity.rs"]
mod jaxb_jaxbtest_b_entity;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/b_id.rs"]
mod jaxb_jaxbtest_b_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/base_root.rs"]
mod jaxb_jaxbtest_base_root;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/c_added_event.rs"]
mod jaxb_jaxbtest_c_added_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/c_entity.rs"]
mod jaxb_jaxbtest_c_entity;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/c_event.rs"]
mod jaxb_jaxbtest_c_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/c_id.rs"]
mod jaxb_jaxbtest_c_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/d_event.rs"]
mod jaxb_jaxbtest_d_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/duplicate_vendor_key_exception.rs"]
mod jaxb_jaxbtest_duplicate_vendor_key_exception;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/impl_root_id.rs"]
mod jaxb_jaxbtest_impl_root_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/jaxb_test_entity_id_factory.rs"]
mod jaxb_jaxbtest_jaxb_test_entity_id_factory;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person.rs"]
mod jaxb_jaxbtest_person;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person_created_event.rs"]
mod jaxb_jaxbtest_person_created_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person_id.rs"]
mod jaxb_jaxbtest_person_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person_name.rs"]
mod jaxb_jaxbtest_person_name;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person_name_changed_event.rs"]
mod jaxb_jaxbtest_person_name_changed_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/person_not_found_exception.rs"]
mod jaxb_jaxbtest_person_not_found_exception;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor.rs"]
mod jaxb_jaxbtest_vendor;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_created_event.rs"]
mod jaxb_jaxbtest_vendor_created_event;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_event_id.rs"]
mod jaxb_jaxbtest_vendor_event_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_example_test.rs"]
mod jaxb_jaxbtest_vendor_example_test;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_id.rs"]
mod jaxb_jaxbtest_vendor_id;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_id_converter.rs"]
mod jaxb_jaxbtest_vendor_id_converter;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_key.rs"]
mod jaxb_jaxbtest_vendor_key;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_key_converter.rs"]
mod jaxb_jaxbtest_vendor_key_converter;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_key_str.rs"]
mod jaxb_jaxbtest_vendor_key_str;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_key_str_validator.rs"]
mod jaxb_jaxbtest_vendor_key_str_validator;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_name.rs"]
mod jaxb_jaxbtest_vendor_name;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_name_converter.rs"]
mod jaxb_jaxbtest_vendor_name_converter;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_name_str.rs"]
mod jaxb_jaxbtest_vendor_name_str;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_name_str_validator.rs"]
mod jaxb_jaxbtest_vendor_name_str_validator;

#[cfg(feature = "xml")]
#[path = "java_parity/jaxb/jaxbtest/vendor_ref.rs"]
mod jaxb_jaxbtest_vendor_ref;

#[path = "java_parity/jsonb/jsonb/abstract_domain_event_test.rs"]
mod jsonb_jsonb_abstract_domain_event_test;

#[path = "java_parity/jsonb/jsonb/abstract_event_test.rs"]
mod jsonb_jsonb_abstract_event_test;

#[path = "java_parity/jsonb/jsonb/aggregate_already_exists_exception_data_test.rs"]
mod jsonb_jsonb_aggregate_already_exists_exception_data_test;

#[path = "java_parity/jsonb/jsonb/aggregate_deleted_exception_data_test.rs"]
mod jsonb_jsonb_aggregate_deleted_exception_data_test;

#[path = "java_parity/jsonb/jsonb/aggregate_not_found_exception_data_test.rs"]
mod jsonb_jsonb_aggregate_not_found_exception_data_test;

#[path = "java_parity/jsonb/jsonb/aggregate_version_conflict_exception_data_test.rs"]
mod jsonb_jsonb_aggregate_version_conflict_exception_data_test;

#[path = "java_parity/jsonb/jsonb/aggregate_version_jsonb_adapter_test.rs"]
mod jsonb_jsonb_aggregate_version_jsonb_adapter_test;

#[path = "java_parity/jsonb/jsonb/aggregate_version_not_found_exception_data_test.rs"]
mod jsonb_jsonb_aggregate_version_not_found_exception_data_test;

#[path = "java_parity/jsonb/jsonb/architecture_test.rs"]
mod jsonb_jsonb_architecture_test;

#[path = "java_parity/jsonb/jsonb/base_test.rs"]
mod jsonb_jsonb_base_test;

#[path = "java_parity/jsonb/jsonb/decryption_failed_exception_data_test.rs"]
mod jsonb_jsonb_decryption_failed_exception_data_test;

#[path = "java_parity/jsonb/jsonb/duplicate_encryption_key_id_exception_data_test.rs"]
mod jsonb_jsonb_duplicate_encryption_key_id_exception_data_test;

#[path = "java_parity/jsonb/jsonb/duplicate_entity_exception_data_test.rs"]
mod jsonb_jsonb_duplicate_entity_exception_data_test;

#[path = "java_parity/jsonb/jsonb/encrypted_data_jsonb_test.rs"]
mod jsonb_jsonb_encrypted_data_jsonb_test;

#[path = "java_parity/jsonb/jsonb/encryption_key_id_unknown_exception_data_test.rs"]
mod jsonb_jsonb_encryption_key_id_unknown_exception_data_test;

#[path = "java_parity/jsonb/jsonb/encryption_key_version_unknown_exception_data_test.rs"]
mod jsonb_jsonb_encryption_key_version_unknown_exception_data_test;

#[path = "java_parity/jsonb/jsonb/entity_id_jsonb_adapter_test.rs"]
mod jsonb_jsonb_entity_id_jsonb_adapter_test;

#[path = "java_parity/jsonb/jsonb/entity_id_path_jsonb_adapter_test.rs"]
mod jsonb_jsonb_entity_id_path_jsonb_adapter_test;

#[path = "java_parity/jsonb/jsonb/entity_not_found_exception_data_test.rs"]
mod jsonb_jsonb_entity_not_found_exception_data_test;

#[path = "java_parity/jsonb/jsonb/event_id_jsonb_adapter_test.rs"]
mod jsonb_jsonb_event_id_jsonb_adapter_test;

#[path = "java_parity/jsonb/jsonb/test_utils.rs"]
mod jsonb_jsonb_test_utils;

#[path = "java_parity/jsonb/jsonbtest/a_created_event.rs"]
mod jsonb_jsonbtest_a_created_event;

#[path = "java_parity/jsonb/jsonbtest/a_id.rs"]
mod jsonb_jsonbtest_a_id;

#[path = "java_parity/jsonb/jsonbtest/a_root.rs"]
mod jsonb_jsonbtest_a_root;

#[path = "java_parity/jsonb/jsonbtest/b_added_event.rs"]
mod jsonb_jsonbtest_b_added_event;

#[path = "java_parity/jsonb/jsonbtest/b_entity.rs"]
mod jsonb_jsonbtest_b_entity;

#[path = "java_parity/jsonb/jsonbtest/b_id.rs"]
mod jsonb_jsonbtest_b_id;

#[path = "java_parity/jsonb/jsonbtest/base_root.rs"]
mod jsonb_jsonbtest_base_root;

#[path = "java_parity/jsonb/jsonbtest/c_added_event.rs"]
mod jsonb_jsonbtest_c_added_event;

#[path = "java_parity/jsonb/jsonbtest/c_entity.rs"]
mod jsonb_jsonbtest_c_entity;

#[path = "java_parity/jsonb/jsonbtest/c_event.rs"]
mod jsonb_jsonbtest_c_event;

#[path = "java_parity/jsonb/jsonbtest/c_id.rs"]
mod jsonb_jsonbtest_c_id;

#[path = "java_parity/jsonb/jsonbtest/d_event.rs"]
mod jsonb_jsonbtest_d_event;

#[path = "java_parity/jsonb/jsonbtest/duplicate_vendor_key_exception.rs"]
mod jsonb_jsonbtest_duplicate_vendor_key_exception;

#[path = "java_parity/jsonb/jsonbtest/impl_root_id.rs"]
mod jsonb_jsonbtest_impl_root_id;

#[path = "java_parity/jsonb/jsonbtest/jsonb_test_entity_id_factory.rs"]
mod jsonb_jsonbtest_jsonb_test_entity_id_factory;

#[path = "java_parity/jsonb/jsonbtest/person.rs"]
mod jsonb_jsonbtest_person;

#[path = "java_parity/jsonb/jsonbtest/person_created_event.rs"]
mod jsonb_jsonbtest_person_created_event;

#[path = "java_parity/jsonb/jsonbtest/person_id.rs"]
mod jsonb_jsonbtest_person_id;

#[path = "java_parity/jsonb/jsonbtest/person_name.rs"]
mod jsonb_jsonbtest_person_name;

#[path = "java_parity/jsonb/jsonbtest/person_name_changed_event.rs"]
mod jsonb_jsonbtest_person_name_changed_event;

#[path = "java_parity/jsonb/jsonbtest/person_not_found_exception.rs"]
mod jsonb_jsonbtest_person_not_found_exception;

#[path = "java_parity/jsonb/jsonbtest/vendor.rs"]
mod jsonb_jsonbtest_vendor;

#[path = "java_parity/jsonb/jsonbtest/vendor_created_event.rs"]
mod jsonb_jsonbtest_vendor_created_event;

#[path = "java_parity/jsonb/jsonbtest/vendor_event_id.rs"]
mod jsonb_jsonbtest_vendor_event_id;

#[path = "java_parity/jsonb/jsonbtest/vendor_example_test.rs"]
mod jsonb_jsonbtest_vendor_example_test;

#[path = "java_parity/jsonb/jsonbtest/vendor_id.rs"]
mod jsonb_jsonbtest_vendor_id;

#[path = "java_parity/jsonb/jsonbtest/vendor_id_jsonb_adapter.rs"]
mod jsonb_jsonbtest_vendor_id_jsonb_adapter;

#[path = "java_parity/jsonb/jsonbtest/vendor_key.rs"]
mod jsonb_jsonbtest_vendor_key;

#[path = "java_parity/jsonb/jsonbtest/vendor_key_jsonb_adapter.rs"]
mod jsonb_jsonbtest_vendor_key_jsonb_adapter;

#[path = "java_parity/jsonb/jsonbtest/vendor_key_str.rs"]
mod jsonb_jsonbtest_vendor_key_str;

#[path = "java_parity/jsonb/jsonbtest/vendor_key_str_validator.rs"]
mod jsonb_jsonbtest_vendor_key_str_validator;

#[path = "java_parity/jsonb/jsonbtest/vendor_name.rs"]
mod jsonb_jsonbtest_vendor_name;

#[path = "java_parity/jsonb/jsonbtest/vendor_name_jsonb_adapter.rs"]
mod jsonb_jsonbtest_vendor_name_jsonb_adapter;

#[path = "java_parity/jsonb/jsonbtest/vendor_name_str.rs"]
mod jsonb_jsonbtest_vendor_name_str;

#[path = "java_parity/jsonb/jsonbtest/vendor_name_str_validator.rs"]
mod jsonb_jsonbtest_vendor_name_str_validator;

#[path = "java_parity/jsonb/jsonbtest/vendor_ref.rs"]
mod jsonb_jsonbtest_vendor_ref;
