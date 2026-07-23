//! `vendor_example_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::architecture_test::TEST_MODEL_BOUNDARY;
use ddd_4_rust_core::AggregateRoot;
use ddd_4_rust_test_model::{PersonName, Vendor, VendorId, VendorKey, VendorName};

/// 执行 `vendor` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn vendor() -> Result<Vendor, Box<dyn std::error::Error>> {
    let mut service = |_: &VendorKey| Ok(());
    Ok(Vendor::new(
        VendorId::new(),
        VendorKey::new("V00001")?,
        VendorName::new("Peter Parker Inc.")?,
        &mut service,
    )?)
}

#[test]
/// 执行 `constructor_emits_vendor_created_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn constructor_emits_vendor_created_event() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(TEST_MODEL_BOUNDARY, "test-model -> core + serde");
    let vendor = vendor()?;
    assert_eq!(vendor.uncommitted_changes().len(), 1);
    assert_eq!(
        vendor.uncommitted_changes()[0].event_type().as_str(),
        "VendorCreatedEvent"
    );
    Ok(())
}

#[test]
/// 执行 `adding_person_emits_person_created_event` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn adding_person_emits_person_created_event() -> Result<(), Box<dyn std::error::Error>> {
    let mut vendor = vendor()?;
    vendor.mark_changes_as_committed();
    let person_id = vendor.add_person(PersonName::new("Peter Parker")?)?;
    assert_eq!(person_id.as_i32(), 1);
    assert_eq!(vendor.uncommitted_changes().len(), 1);
    assert_eq!(
        vendor.uncommitted_changes()[0].event_type().as_str(),
        "PersonCreatedEvent"
    );
    Ok(())
}

#[test]
/// 执行 `changing_person_name_emits_old_and_new_values` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn changing_person_name_emits_old_and_new_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut vendor = vendor()?;
    vendor.mark_changes_as_committed();
    let person_id = vendor.add_person(PersonName::new("Peter Parker")?)?;
    vendor.mark_changes_as_committed();
    vendor.change_person_name(person_id, PersonName::new("Harry Osborn")?)?;
    assert_eq!(vendor.uncommitted_changes().len(), 1);
    assert_eq!(
        vendor.uncommitted_changes()[0].event_type().as_str(),
        "PersonNameChangedEvent"
    );
    assert_eq!(vendor.persons()[0].name().as_str(), "Harry Osborn");
    Ok(())
}
